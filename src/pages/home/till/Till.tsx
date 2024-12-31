import React, {
  ChangeEvent,
  Children,
  Component,
  FormEvent,
  useContext,
  useRef,
  useState,
} from "react";
import NavBar from "../components/NavBar";
import { DateTime } from "luxon";
import PersonIcon from "../../../assets/person.svg";
import ProceedIcon from "../../../assets/proceed.svg";
import ListIcon from "../../../assets/list.svg";
import PlayIcon from "../../../assets/play-line.svg";
import CloseIcon from "../../../assets/close.svg";
import HoldIcon from "../../../assets/hold.svg";
import AddIcon from "../../../assets/add_person.svg";
import BackArrowIcon from "../../../assets/back_arrow.svg";
import img from "../../../assets/nathalia-rosa-rWMIbqmOxrY-unsplash.jpg";
import ReprintIcon from "../../../assets/reprint.svg";
import { invoke } from "@tauri-apps/api/core";
import "./till.scss";
import { useSnackbar } from "notistack";
import { Staff } from "../../auth/Login";
("tauri-plugin-printer");

import {
  ENTITY_KEY,
  STAFF_KEY,
  transactions as TRANSACTIONS,
  TRX,
} from "../../../utils/constants";
import { Entity } from "../../auth/LoadEntity";
import { CartIcon, IPayment, TransactionsDialog } from "./componets";
import { CartItem } from "./componets";
import { CheckoutDialog } from "./componets";
import { PausedTransactionsDialog } from "./componets";
import { useAppDispatch, useAppSelector } from "../../../redux/hooks";
import { setReceipt } from "../../../redux/reducers/receiptReducer";

export interface IProduct {
  _id: string;
  name: string;
  quantity: number;
  category: string;
  barcode?: string;
  reorderLimit?: number;
  sell_price: number;
  cost_price: number;
  expiry_date: string;
  total_profit?: number;
  quantity_sold?: number;
  refunded_quantity?: number;
  refunded_amount?: number;
  entityId: string;
  isArchived: boolean;
  isBalanced: boolean;
  createdAt: string;
  updatedAt: string;
  __v: number;
}
export interface ICartItem {
  product: IProduct;
  quantity: number;
  discount: number;
  total: number;
}
export interface ITransactionItem {
  discount: number;
  entityId: string;
  invoiceNumber: string;
  items: Item[];
  mop: string;
  profit: number;
  staff: string;
  staffId: number;
  terminal: string;
  total: number;
  change: number;
  createdAt?: string;
  customer?: number;
  transaction_id?: number;
  payments: IPayment[];
}
export interface Item {
  quantity: number;
  product_id: string;
  product?: IProduct;
  name: string;
  discount: number;
  sell_price: number;
  profit: number;
  total: number;
}
function Till() {
  const [cart, setCart] = useState<ICartItem[]>([]);

  const searchRef = useRef<HTMLInputElement>(null);

  const [total, setTotal] = useState<number>(0);

  const [isVisible, setIsVisible] = useState(false);

  const [showTransactions, setShowTransactions] = useState(false);

  const [showPaused, setShowPaused] = useState(false);

  const [discount, setDiscount] = useState<number>(0);

  const { enqueueSnackbar: notify } = useSnackbar();

  const [products, setProducts] = useState<IProduct[]>([]);

  const [showAddCustomer, setShowAddCustomer] = useState(false);

  const entity = useAppSelector((state) => state.entity);
  const receipt = useAppSelector((state) => state.receipt);
  const staff = useAppSelector((state) => state.user);
  const dispatch = useAppDispatch();
  const [customer, setCustomer] = useState<ICustomer>();
  const handleChange = async (e: ChangeEvent<HTMLInputElement>) => {
    let value = e.target.value.trim();
    if (value.length > 0) {
      try {
        let products: IProduct[] = await invoke("search_products", {
          queryString: value,
        });
        setProducts(products);
        if (products.length < 1)
          notify(`No product found for ${value}`, { variant: "info" });
      } catch (error) {
        console.log(error);
      }
    } else setProducts([]);
  };
  const saveCart = (cart: ICartItem[]) => {
    localStorage.setItem("cart", JSON.stringify(cart));
  };
  const handleAddToCart = (product: IProduct) => {
    if (cart.find((e) => e.product._id === product._id))
      return notify(`${product.name} is already in cart`, {
        variant: "warning",
      });
    if (searchRef.current) {
      searchRef.current.value = "";
      searchRef.current.focus();
    }
    if (product.reorderLimit && product.quantity <= product?.reorderLimit) {
      notify(product.name + "'s quantity is below limit", {
        variant: "info",
        autoHideDuration: 10000,
      });
    }
    setProducts([]);
    setCart((state) => {
      let newCart = [
        ...state,
        { product, quantity: 1, discount: 0, total: product.sell_price },
      ];
      saveCart(newCart);
      handleUpdateFigures(newCart);
      return newCart;
    });
  };

  const handleUpdateCartByClick = (
    mode: string,
    cartItem: ICartItem,
    index: number,
    setValue: React.Dispatch<React.SetStateAction<number>>,
    value: number
  ) => {
    switch (mode) {
      case "-": {
        if (cartItem.quantity === 1) return;
        cartItem.quantity -= 1;
        cartItem.total = cartItem.quantity * cartItem.product.sell_price;
        let updatedCart = [...cart];
        updatedCart[index] = cartItem;
        setCart(updatedCart);
        saveCart(updatedCart);
        handleUpdateFigures(updatedCart);
        break;
      }

      case "+": {
        cartItem.quantity += 1;
        cartItem.total = cartItem.quantity * cartItem.product.sell_price;
        let updatedCart = [...cart];
        updatedCart[index] = cartItem;
        setCart(updatedCart);
        saveCart(updatedCart);
        handleUpdateFigures(updatedCart);
        break;
      }
      default:
        "";
    }
  };

  const handleUpdateCartByChange = (
    value: number,
    cartItem: ICartItem,
    index: number
  ) => {
    cartItem.quantity = value;
    cartItem.total = cartItem.quantity * cartItem.product.sell_price;
    let updatedCart = [...cart];
    updatedCart[index] = cartItem;
    setCart(updatedCart);
    saveCart(updatedCart);
    handleUpdateFigures(updatedCart);
  };

  const handleDiscount = (
    value: number,
    cartItem: ICartItem,
    index: number
  ) => {
    if (value >= cartItem.product.sell_price * cartItem.quantity)
      return notify("This does not make sense", { variant: "warning" });
    cartItem.discount = value;
    // let discount = 0;
    // cart.forEach((item) => (discount += item.discount));
    let updatedCart = [...cart];
    updatedCart[index] = cartItem;
    setCart(updatedCart);
    saveCart(updatedCart);
    //setDiscount(discount);
    handleUpdateFigures(updatedCart);
  };

  const handleDelete = (cartItem: ICartItem) => {
    let updatedCart = [
      ...cart.filter((item) => item.product._id !== cartItem.product._id),
    ];

    setCart(updatedCart);
    saveCart(updatedCart);
    handleUpdateFigures(updatedCart);
  };

  const handleUpdateFigures = (cart: ICartItem[]) => {
    let total = 0;
    let discount = 0;
    cart.forEach((item) => {
      discount += item.discount;
      total += item.total;
    });
    setDiscount(discount);
    setTotal(total - discount);
  };
  const handleResetFigures = () => {
    setDiscount(0);
    setTotal(0);
    setCart([]);
    localStorage.setItem("cart", JSON.stringify([]));
  };

  const handleCloseDialog = () => {
    setIsVisible(false);
  };
  const handlePostTransaction = async (
    change: number,
    payments: IPayment[]
  ) => {
    let profit = 0;
    let now = DateTime.now();
    let rawCount = localStorage.getItem("count");
    if (!rawCount) return;
    let count = JSON.parse(rawCount);
    let invoiceNumber = `${count.count}${now.month}${now.day}${Math.round(
      Math.random() * now.year + count.count
    )}`;
    let items: Item[] = cart.map(({ quantity, discount, total, product }) => {
      let unitProfit =
        product.sell_price * quantity - product.cost_price * quantity;
      profit += unitProfit;
      return {
        quantity,
        discount,
        total,
        product_id: product._id,
        name: product.name,
        sell_price: product.sell_price,
        profit: unitProfit,
      };
    });

    let transaction: ITransactionItem = {
      discount,
      total: total,
      change,
      mop: [...payments.map((p) => p.name)].join(", "),
      entityId: cart[0].product.entityId,
      items,
      terminal: entity?.terminal ?? "",
      payments,
      profit,
      staff: staff?.username ?? "",
      staffId: staff?.staffId ?? 0,
      customer: customer?.customerId,
      invoiceNumber,
      transaction_id: 0,
    };
    localStorage.setItem(
      "count",
      JSON.stringify({ ...count, count: count.count + 1 })
    );
    dispatch(
      setReceipt({
        ...entity,
        discount: transaction.discount,
        total: transaction.total,
        modeOfPayment: transaction.mop,
        showSideBar: false,
        entity,
        invoiceNumber,
        items,
        staff: transaction.staff,
        customer: customer?.fullName,
      })
    );
    delete transaction.transaction_id;
    console.log(transaction);
    try {
      let result = await invoke("save_transaction", {
        transaction: {
          ...transaction,
          is_synced: false,
        },
      });
      if (result === true) {
        localStorage.setItem(TRX, JSON.stringify(transaction));
        window.print();
        notify("Done", { variant: "success" });
        setCart([]);
        setDiscount(0);
        setTotal(0);
        setIsVisible(false);
        searchRef.current?.focus();
      }
    } catch (error) {
      console.error(error);
      notify(error, { variant: "error" });
    }
  };
  const holdOrder = () => {
    if (cart.length < 1) return notify("Hold what? air?");
    let heldOrders = localStorage.getItem(TRANSACTIONS)
      ? JSON.parse(localStorage.getItem(TRANSACTIONS)!)
      : [{ transactions: [], total: 0, discount: 0 }];
    localStorage.setItem(
      TRANSACTIONS,
      JSON.stringify([{ transactions: cart, total, discount }, ...heldOrders])
    );
    notify("Order on hold", { variant: "info" });
    setCart([]);
    setDiscount(0);
    setTotal(0);
    setProducts([]);
  };
  const handleRestoreCart = ({
    total,
    discount,
    transactions,
  }: {
    transactions: ICartItem[];
    total: number;
    discount: number;
  }) => {
    setCart(transactions);
    setDiscount(discount);
    setTotal(total);
    setShowPaused(false);
    setShowTransactions(false);
  };
  const handleClosePausedDialog = () => {
    setShowPaused(false);
  };

  const handleSelectCustomer = (customer: ICustomer | undefined) => {
    setCustomer(customer);
    setShowAddCustomer(false);
  };

  return (
    <div className="till-con">
      <img src={img} alt="" />
      <NavBar
        onClick={handleAddToCart}
        onChange={handleChange}
        products={products}
        searchRef={searchRef}
      />
      {isVisible && (
        <CheckoutDialog
          closeDialog={handleCloseDialog}
          isVisible={isVisible}
          total={total}
          discount={discount}
          postTransaction={handlePostTransaction}
          pm={entity?.paymentMethods ?? []}
          customer={customer}
        />
      )}

      {showPaused && (
        <PausedTransactionsDialog
          closeDialog={handleClosePausedDialog}
          restoreCart={handleRestoreCart}
        />
      )}
      {showTransactions && (
        <TransactionsDialog
          restoreCart={handleRestoreCart}
          closeDialog={() => setShowTransactions(false)}
        />
      )}
      {showAddCustomer && <AddCustomer selectCustomer={handleSelectCustomer} />}
      <section className="till-con__section">
        <div className="till-con__section__favorites"></div>
        <div className="till-con__section__cart">
          <div className="till-con__section__cart__header">
            <CartIcon
              label="Add Customer"
              Icon={PersonIcon}
              onClick={() => {
                setShowAddCustomer(true);
              }}
            />
            <CartIcon label="Print Quote" Icon="" onClick={() => {}} />
            <CartIcon
              label="Transactions"
              Icon={ListIcon}
              onClick={() => {
                setShowTransactions(true);
              }}
            />
            <CartIcon
              label="Resume Orders"
              Icon={PlayIcon}
              onClick={() => {
                setShowPaused(true);
              }}
            />
            <CartIcon
              label="Reprint"
              Icon={ReprintIcon}
              onClick={() => {
                // let rawTransaction = localStorage.getItem(TRX);
                // let rawEntity = localStorage.getItem(ENTITY_KEY);
                // if (rawTransaction && rawEntity) {
                //   let entity: Entity = JSON.parse(rawEntity);
                //   let transaction: ITransactionItem =
                //     JSON.parse(rawTransaction);
                //   context?.setContext({
                //     discount: transaction.discount,
                //     total: transaction.total,
                //     mop: transaction.mop,
                //     showSideBar: false,
                //     entity,
                //     invoiceNumber: transaction.invoiceNumber,
                //     items: transaction.items,
                //     staff: transaction.staff,
                //   });
                // }
                window.print();
              }}
            />
            <CartIcon
              label="Delete All"
              Icon={ReprintIcon}
              onClick={async () => {
                let shouldDelete = await confirm("Delete all items from cart?");
                console.log(shouldDelete);

                if (shouldDelete) handleResetFigures();
              }}
            />
          </div>

          <div className="till-con__section__cart__items">
            <div className="till-con__section__cart__items__item__header">
              <i></i>
              <p>#</p>
              <p>Name</p>
              <p>Qty</p>
              <p>&#8358;Price</p>
              <p>&#8358;Total</p>
            </div>

            {cart.map((cartItem, index) => (
              <CartItem
                key={cartItem.product._id}
                onChange={handleUpdateCartByChange}
                onClick={handleUpdateCartByClick}
                cartItem={cartItem}
                discountChange={handleDiscount}
                index={index}
                handleDelete={handleDelete}
              />
            ))}
          </div>
          <div className="till-con__section__cart__summary-con">
            {customer && (
              <div className="till-con__section__cart__summary-con__summary customer">
                <p>Customer</p>
                <p>
                  {" "}
                  <span>{customer?.fullName}</span>{" "}
                  <span onClick={() => setCustomer(undefined)}>
                    <CloseIcon />
                  </span>{" "}
                </p>
              </div>
            )}
            <div className="till-con__section__cart__summary-con__summary">
              <p>Subtotal</p>
              <p>&#8358; {(total + discount).toLocaleString()}</p>
            </div>
            <div className="till-con__section__cart__summary-con__summary">
              <p>Discount</p>
              <p>- &#8358; {discount.toLocaleString()}</p>
            </div>
            <div className="till-con__section__cart__summary-con__summary">
              <p>Payable Amount</p>
              <p>&#8358; {total.toLocaleString()}</p>
            </div>
          </div>
          <div className="till-con__section__cart__options">
            <div
              onClick={() => holdOrder()}
              className="till-con__section__cart__options__option"
            >
              <HoldIcon /> <p>Hold Order</p>
            </div>
            <div
              onClick={() => {
                if (cart.length < 1)
                  return notify("Proceed with what?", { variant: "success" });
                setIsVisible(true);
              }}
              className="till-con__section__cart__options__option"
            >
              <ProceedIcon /> <p>Proceed</p>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
export interface ICustomer {
  fullName: string;
  firstName: string;
  lastName: string;
  phone: string;
  wallet: number;
  totalRevenue: number;
  customerId: number;
}

const AddCustomer = ({
  selectCustomer,
}: {
  selectCustomer: (customer: ICustomer | undefined) => void;
}) => {
  const [showForm, setShowForm] = useState(false);
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [customers, setCustomers] = useState<ICustomer[]>([]);
  const { enqueueSnackbar: notify } = useSnackbar();
  const [phone, setPhone] = useState("");
  let conRef = useRef<HTMLDivElement>(null);
  const handleSubmit = async (e: FormEvent) => {
    try {
      e.preventDefault();
      let request: ICustomer = await invoke("save_customer", {
        firstName,
        lastName,
        phone,
      });
      if (request) {
        notify(`${request.firstName} ${request.lastName} added as customer`);
        setFirstName("");
        setLastName("");
        setPhone("");
      }
    } catch (error) {
      console.log(error);
    }
  };
  const handleSearch = async (e: ChangeEvent) => {
    let { value } = e.target as HTMLInputElement;
    if (value.trim()) {
      let customers: ICustomer[] = await invoke("get_customers", {
        searchQuery: value,
      });
      setCustomers(customers);
    }
  };
  return (
    <div
      onClick={(e) => {
        let contains = (e.target as HTMLDivElement).contains(conRef.current);
        if (contains) selectCustomer(undefined);
      }}
      className="add-customer-con"
    >
      <div ref={conRef} className="add-customer">
        {" "}
        {showForm ? (
          <form onSubmit={handleSubmit} className="add-customer-con__form">
            <div className="form-header">
              <div
                className="svg-btn"
                onClick={() => {
                  setShowForm(false);
                }}
              >
                <BackArrowIcon />
              </div>{" "}
              <h3>Add Customer</h3>{" "}
            </div>
            <div className="form-con">
              <div className="add-customer-con__form__input-con">
                {" "}
                <label htmlFor="">First Name</label>
                <input
                  value={firstName}
                  onChange={(e) => setFirstName(e.target.value)}
                  type="text"
                />{" "}
              </div>
              <div className="add-customer-con__form__input-con">
                {" "}
                <label htmlFor="">Last Name</label>
                <input
                  onChange={(e) => setLastName(e.target.value)}
                  type="text"
                  value={lastName}
                />{" "}
              </div>
              <div className="add-customer-con__form__input-con">
                {" "}
                <label htmlFor="">Phone Number</label>
                <input
                  onChange={(e) => setPhone(e.target.value)}
                  type="text"
                  value={phone}
                />{" "}
              </div>
              <div className="add-customer-con__form__input-con">
                <label htmlFor="Name"></label>
                <button>Save Customer</button>{" "}
              </div>
            </div>
          </form>
        ) : (
          <>
            <div className="add-customer__input-con">
              <input
                type="search"
                onChange={handleSearch}
                placeholder="Search Customers"
              />
              <div
                className="svg-btn"
                onClick={() => {
                  setShowForm(true);
                }}
              >
                {" "}
                <AddIcon />
              </div>
            </div>
            {customers.length > 0 ? (
              <div className="add-customer-body">
                {" "}
                {customers.map((customer) => (
                  <p onClick={() => selectCustomer(customer)}>
                    {customer.fullName} {customer.phone}
                  </p>
                ))}
              </div>
            ) : (
              <div className="empty-body">
                <p>Select a customer or add a new customer to continue</p>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
};

export default Till;
