import React, { useContext, useEffect, useState } from "react";
import ArrowIcon from "../../../assets/arrow.svg";
import ArrowLongIcon from "../../../assets/arrow-long.svg";
import CloseIcon from "../../../assets/close.svg";
import MinusIcon from "../../../assets/minus.svg";
import AddIcon from "../../../assets/plus.svg";
import SettingsIcon from "../../../assets/settings.svg";
import {
  ENTITY_KEY,
  transactions as TRANSACTIONS,
} from "../../../utils/constants";
import { ICartItem, ICustomer, ITransactionItem } from "./Till";
import { invoke } from "@tauri-apps/api/core";
import { Entity, PaymentMethod } from "../../auth/LoadEntity";
import { useAppDispatch, useAppSelector } from "../../../redux/hooks";
import { setReceipt } from "../../../redux/reducers/receiptReducer";
import { useSnackbar } from "notistack";

export interface IPayment {
  name: string;
  _id: string;
  value: number;
  transactionId?: number;
  payment_method_id: string;
}
export const CartIcon = ({
  label,
  onClick,
  Icon,
}: {
  label: string;
  Icon?: string;
  onClick: () => void;
}) => (
  <div
    onClick={() => onClick()}
    className="till-con__section__cart__header__icon"
  >
    {Icon && <Icon />} <p>{label}</p>
  </div>
);
export const CartItem = ({
  cartItem,
  index,
  onClick,
  onChange,
  discountChange,
  handleDelete,
}: {
  cartItem: ICartItem;
  index: number;
  onChange: (value: number, cartItem: ICartItem, index: number) => void;
  discountChange: (value: number, cartItem: ICartItem, index: number) => void;
  handleDelete: (cartItem: ICartItem) => void;
  onClick: (
    mode: string,
    cartItem: ICartItem,
    index: number,
    setValue: React.Dispatch<React.SetStateAction<number>>,
    value: number
  ) => void;
}) => {
  let [value, setValue] = useState(1);
  let [isActive, setIsActive] = useState(false);

  return (
    <div className="till-con__section__cart__items__item">
      <div
        onClick={(e) => {
          setIsActive(!isActive);
        }}
        className={
          "till-con__section__cart__items__item__header " +
          (isActive && " active")
        }
      >
        <ArrowIcon />
        <p>{index + 1}</p>
        <p>{cartItem.product.name}</p>
        <p>{cartItem.quantity.toLocaleString()}</p>
        <p>{cartItem.product.sell_price.toLocaleString()}</p>
        <p>{cartItem.total.toLocaleString()}</p>
      </div>
      <div
        className={
          "till-con__section__cart__items__item__options " +
          (isActive && " active")
        }
      >
        <div className="quantity-con">
          <small>Quantity</small>
          <div
            onClick={(e) => {
              e.stopPropagation();
              onClick("-", cartItem, index, setValue, value);
            }}
            className="icon-con"
          >
            <MinusIcon />
          </div>
          <input
            onChange={(e) => {
              e.stopPropagation();
              setValue(+e.target.value);
              onChange(+e.target.value, cartItem, index);
            }}
            type="number"
            value={cartItem.quantity}
          />
          <div
            onClick={(e) => {
              e.stopPropagation();
              onClick("+", cartItem, index, setValue, value);
            }}
            className="icon-con"
          >
            <AddIcon />
          </div>
        </div>{" "}
        <div className="discount-con">
          <small>Discount</small>
          <input
            type="number"
            onChange={(e) => discountChange(+e.target.value, cartItem, index)}
            value={cartItem.discount}
          />
        </div>
        <div onClick={() => handleDelete(cartItem)} className="delete">
          <CloseIcon />
        </div>
      </div>
    </div>
  );
};
export const CheckoutDialog = ({
  total,
  discount,
  isVisible,
  closeDialog,
  postTransaction,
  pm,
  customer,
}: {
  total: number;
  isVisible: boolean;
  discount: number;
  closeDialog: () => void;
  postTransaction: (change: number, payments: IPayment[]) => void;
  pm: PaymentMethod[];
  customer: ICustomer | undefined;
}) => {
  const [change, setChange] = useState(0);
  let [totalPayment, setTotalPayment] = useState(0);
  let [moneyReceived, setMoneyReceived] = useState(0);
  const { enqueueSnackbar: notify } = useSnackbar();
  const [showPaymentSettings, setShowPaymentSettings] = useState(false);
  let [paymentMethods, setPaymentMethods] = useState(pm);
  const [payments, setPayments] = useState<IPayment[]>([]);
  const handleUpdateChange = (value: string) => {
    if (value && +value > total) {
      setChange(+value - total);
      console.log(total);
    }
  };
  return (
    <div className={"checkout-dialog-con " + (isVisible ? "visible" : "")}>
      <div className={"checkout-dialog " + (isVisible ? "visible-slide" : "")}>
        <div
          className={"payment-settings " + (showPaymentSettings && "slide-in")}
        >
          <div className="payment-settings__header">
            <button
              onClick={() => {
                setShowPaymentSettings(false);
                setPayments([]);
                setTotalPayment(0);
                setPaymentMethods((state) =>
                  state.map((p) => {
                    return { ...p, isActive: false, value: 0 };
                  })
                );
              }}
            >
              <ArrowLongIcon />
            </button>{" "}
            <h3>Payment Settings</h3> <p>{total - totalPayment}</p>
          </div>
          <div className="payment-settings__body">
            {paymentMethods.map((paymentMethod, index) => (
              <PaymentMethodComp
                key={paymentMethod._id}
                paymentMethod={paymentMethod}
                onChange={(v) => {
                  let clone = { ...paymentMethod, value: v };
                  let pms = [...paymentMethods];
                  pms[index] = clone;
                  setPaymentMethods(pms);
                  let pm = [...payments].find(
                    (pm) => pm._id == paymentMethod._id
                  );

                  if (pm !== undefined && pm) {
                    let clone = { ...pm, value: v };
                    let newPayments = [
                      ...payments.filter((p) => p._id !== pm._id),
                      clone,
                    ];
                    let totalPayment = newPayments
                      .map((p) => p.value)
                      .reduce((a, b) => a + b);

                    setPayments(newPayments);
                    setTotalPayment(totalPayment);
                  }
                }}
                onCheck={(p) => {
                  let clone = { ...p, isActive: !p.isActive };
                  let pms = [...paymentMethods];
                  pms[index] = clone;
                  setPaymentMethods(pms);
                  payments.find((pm) => p._id === pm._id)
                    ? setPayments((state) =>
                        state.filter((pm) => pm._id !== p._id)
                      )
                    : setPayments((state) => [
                        ...state,
                        {
                          ...p,
                          value: p.value ? p.value : 0,
                          payment_method_id: p._id,
                        },
                      ]);
                }}
              />
            ))}
          </div>
          <button
            onClick={() => {
              if (totalPayment < total) {
                notify("Payment cannot be less than amount payable", {
                  variant: "warning",
                });
                return;
              }
              setShowPaymentSettings(false);
            }}
            className="ok"
          >
            Save
          </button>
        </div>
        <div className="checkout-dialog__header">
          <h3>Checkout {customer && "for " + customer.fullName}</h3>
          <div
            onClick={() => {
              setChange(0);
              closeDialog();
            }}
            className="close-dialog"
          >
            <CloseIcon />
          </div>
        </div>

        <p className="total">&#8358;{total.toLocaleString()}</p>

        <form
          onSubmit={(e) => {
            e.preventDefault();
            postTransaction(change, [
              {
                ...paymentMethods[0],
                value: total,
                payment_method_id: paymentMethods[0]._id,
              },
            ]);
          }}
          className="input-con"
        >
          <label htmlFor="">Amount Received</label>
          <input
            type="number"
            onChange={(e) => handleUpdateChange(e.target.value)}
            defaultValue={total}
            placeholder="Amount Received"
            autoFocus={true}
          />
        </form>
        <div className="input-con">
          <label htmlFor="">Change Due</label>
          <p>&#8358;{change.toLocaleString()}</p>
        </div>
        <div className="input-con">
          <label htmlFor="">Total Discount</label>
          <p>-&#8358;{discount.toLocaleString()}</p>
        </div>

        <div className="mode-of-payment-con">
          <label>Payment Method</label>

          {payments.length > 0 ? (
            <div className="payment-display">
              <button
                onClick={() => {
                  setPayments([]);
                  setPaymentMethods((state) =>
                    state.map((p) => {
                      return { ...p, isActive: false, value: 0 };
                    })
                  );
                  setChange(0);
                  setTotalPayment(0);
                }}
              >
                <CloseIcon />
              </button>
              {payments.map((payment) => (
                <p>
                  {payment.name} {payment.value}
                </p>
              ))}
            </div>
          ) : (
            <select
              onChange={(e) => {
                let id = e.target.value;
                let paymentMethod = paymentMethods.find(
                  (paymentMethod) => paymentMethod._id == id
                );
                if (paymentMethod)
                  setPayments([
                    { ...paymentMethod, value: total, payment_method_id: id },
                  ]);
              }}
              name="payment_method"
              id=""
            >
              {paymentMethods.map((payment_method) => (
                <option key={payment_method._id} value={payment_method._id}>
                  {payment_method.name}
                </option>
              ))}
            </select>
          )}
          <button onClick={() => setShowPaymentSettings(true)}>
            <SettingsIcon />
          </button>
        </div>
        <button
          className="checkout-btn"
          onClick={() =>
            postTransaction(
              change,
              payments.length < 1
                ? [
                    {
                      ...paymentMethods[0],
                      value: total,
                      payment_method_id: paymentMethods[0]._id,
                    },
                  ]
                : payments
            )
          }
        >
          Check Out
        </button>
      </div>
    </div>
  );
};

const PaymentMethodComp = ({
  paymentMethod,
  onCheck,
  onChange,
}: {
  onChange: (v: number) => void;
  paymentMethod: PaymentMethod;
  onCheck: (mop: PaymentMethod) => void;
}) => {
  return (
    <div
      tabIndex={1}
      onClick={(e) => {
        onCheck(paymentMethod);
        //setIsActive(!isActive);
      }}
      className={
        (paymentMethod.isActive ? "active " : "") +
        "payment-settings__body__input-con"
      }
    >
      <label>{paymentMethod.name}</label>
      <input
        onClick={(e) => e.stopPropagation()}
        onChange={(e) => onChange(+e.target.value)}
        disabled={!paymentMethod.isActive}
        type="number"
        value={paymentMethod.value ?? ""}
      />
    </div>
  );
};
export const PausedTransactionsDialog = ({
  restoreCart,
  closeDialog,
}: {
  restoreCart: (transactions: {
    transactions: ICartItem[];
    total: number;
    discount: number;
  }) => void;
  closeDialog: () => void;
}) => {
  const [transactions, setTransactions] = useState<
    {
      transactions: ICartItem[];
      total: number;
      discount: number;
    }[]
  >([]);
  const [activeTransaction, setActiveTransaction] = useState<{
    transactions: ICartItem[];
    total: number;
    discount: number;
    index: number;
  }>({ transactions: [], total: 0, discount: 0, index: 0 });
  useEffect(() => {
    let trxs = JSON.parse(localStorage.getItem(TRANSACTIONS) ?? "[]");

    setTransactions(trxs);
  }, []);
  return (
    <dialog
      open
      onClick={(e) => {
        let target = e.target as HTMLDialogElement;
        if (!target.closest(".paused-transaction-con")) closeDialog();
      }}
    >
      <div className="paused-transaction-con">
        <div className="paused-transaction-con__transaction">
          <div className="paused-transaction-con__transaction__card">
            {transactions.map((transaction, index) => (
              <div
                onClick={() => setActiveTransaction({ ...transaction, index })}
                key={index}
                className="paused-transaction-con__card"
              >
                <p>{transaction.transactions[0]?.product.name}</p>
                <p className="paused-transaction-con__card__total">
                  <span
                    onClick={(e) => {
                      e.stopPropagation();
                      let filtered = transactions.filter(
                        (trx, i) => index !== i
                      );
                      setTransactions(filtered);
                      localStorage.setItem(
                        TRANSACTIONS,
                        JSON.stringify(filtered)
                      );
                    }}
                  >
                    <CloseIcon />
                  </span>
                  <span className="total">
                    &#8358;
                    {(
                      transaction.total - transaction.discount
                    ).toLocaleString()}
                  </span>
                </p>
              </div>
            ))}
          </div>
        </div>
        <div className="paused-transaction-con__detail">
          {transactions.length === 0 ? (
            <div className="empty-transaction">No Paused transactions</div>
          ) : activeTransaction.transactions.length == 0 ? (
            <div className="empty-transaction">
              Select a transaction from the list
            </div>
          ) : (
            <div className="paused-transaction-con__detail__con">
              <div className="paused-transaction-con__detail__con__table-con">
                <table>
                  {" "}
                  <thead>
                    {" "}
                    <tr>
                      {" "}
                      <th>#</th>
                      <th>Name</th>
                      <th>Qty</th>
                      <th>Price</th>
                      <th>Total</th>
                    </tr>
                  </thead>
                  <tbody>
                    {activeTransaction.transactions.map(
                      ({ quantity, product, total }, index) => (
                        <tr>
                          <td>{index + 1}</td>
                          <td>{product.name}</td>
                          <td>{quantity}</td>
                          <td>&#8358;{product.sell_price?.toLocaleString()}</td>
                          <td>&#8358;{total.toLocaleString()}</td>
                        </tr>
                      )
                    )}
                  </tbody>
                </table>
              </div>

              <div className="detail-row">
                <p>Discount</p>
                <p>-{activeTransaction.discount?.toLocaleString()}</p>
              </div>
              <div className="detail-row">
                <p>Total</p>
                <p className="detail-row__total">
                  &#8358;
                  {(
                    activeTransaction.total - activeTransaction.discount
                  )?.toLocaleString()}
                </p>
              </div>

              <button
                onClick={(e) => {
                  e.stopPropagation();
                  let filtered = transactions.filter(
                    (trx, i) => activeTransaction.index !== i
                  );
                  setTransactions(filtered);
                  localStorage.setItem(TRANSACTIONS, JSON.stringify(filtered));
                  restoreCart(activeTransaction);
                }}
                className="detail-row__button"
              >
                Back to Till
              </button>
            </div>
          )}
        </div>
      </div>
    </dialog>
  );
};

export const TransactionsDialog = ({
  restoreCart,
  closeDialog,
}: {
  restoreCart: (transactions: {
    transactions: ICartItem[];
    total: number;
    discount: number;
  }) => void;
  closeDialog: () => void;
}) => {
  const [transactions, setTransactions] = useState<ITransactionItem[] | null>(
    null
  );
  const entity = useAppSelector((state) => state.entity);
  const dispatch = useAppDispatch();
  const receipt = useAppSelector((state) => state.receipt);
  let [transaction, setTransaction] = useState<ITransactionItem | null>(null);

  const getTrx = async () => {
    try {
      let trxs: ITransactionItem[] = await invoke("get_transactions");
      setTransactions(trxs);
    } catch (error) {
      console.log(error);
    }
  };
  let search_transactions = async (searchQuery: string) => {
    try {
      let res: ITransactionItem[] = await invoke("search_transactions", {
        searchQuery,
      });
      setTransactions(res);
      console.log(res);
    } catch (error) {
      console.log("====================================");
      console.log(error);
      console.log("====================================");
    }
  };

  const handlePrint = async () => {
    dispatch(
      setReceipt({
        ...entity,
        discount: transaction?.discount,
        total: transaction?.total,
        mop: transaction?.mop,
        showSideBar: false,
        entity,
        invoiceNumber: transaction?.invoiceNumber,
        items: transaction?.items,
        staff: transaction?.staff,
      })
    );
    window.print();
  };

  useEffect(() => {
    getTrx();
  }, []);

  async function handleBackToTill(trx: ITransactionItem) {
    await invoke("refund_transaction", { transaction: trx });
    let trxs = trx.items.map(({ discount, total, product, quantity }) => {
      let cartItem: ICartItem = {
        discount,
        total,
        product: product!,
        quantity,
      };
      return cartItem;
    });
    restoreCart({
      total: trx.total,
      discount: trx.discount,
      transactions: trxs,
    });
  }

  return (
    <dialog
      onClick={(e) => {
        let target = e.target as HTMLDialogElement;
        if (!target.closest(".transactions-con__section")) closeDialog();
      }}
      className="transactions-con"
    >
      <section className="transactions-con__section">
        <div className="transactions-con__section__header">
          <p>TRANSACTIONS</p>
        </div>
        <div className="transactions-con__section__main">
          <div className="transactions-con__section__main__invoices">
            <div className="search-bar">
              <input
                onChange={(e) => {
                  search_transactions(e.target.value);
                }}
                type="text"
                placeholder="Search transactions"
              />
            </div>
            {transactions?.map((trx) => (
              <div
                onClick={() => {
                  console.log(trx);

                  setTransaction(trx);
                }}
                className={
                  "transactions-con__section__main__invoices__invoice" +
                  (trx.invoiceNumber == transaction?.invoiceNumber
                    ? " clicked"
                    : "")
                }
              >
                <p>
                  {trx.invoiceNumber}{" "}
                  <span> &#8358;{trx.total.toLocaleString()}</span>
                </p>
                <p>
                  <span>{trx.createdAt}</span>
                </p>
              </div>
            ))}
          </div>
          <div className="transactions-con__section__main__details">
            {!transaction ? (
              <h1>Select a transaction</h1>
            ) : (
              <>
                {" "}
                <h3>Invoice-{transaction.invoiceNumber}</h3>
                <div className="transactions-con__section__main__details__table-con">
                  <table>
                    {" "}
                    <thead>
                      {" "}
                      <tr>
                        {" "}
                        <th>#</th>
                        <th>Name</th>
                        <th>Qty</th>
                        <th>Price</th>
                        <th>Discount</th>
                        <th>Total</th>
                      </tr>
                    </thead>
                    <tbody>
                      {transaction?.items.map(
                        (
                          { quantity, name, total, sell_price, discount },
                          index
                        ) => (
                          <tr>
                            <td>{index + 1}</td>
                            <td>{name}</td>
                            <td>{quantity}</td>
                            <td>&#8358;{sell_price?.toLocaleString()}</td>
                            <td>&#8358;{discount?.toLocaleString()}</td>
                            <td>&#8358;{total.toLocaleString()}</td>
                          </tr>
                        )
                      )}
                    </tbody>
                  </table>
                </div>
                <div className="transactions-con__section__main__details__summary">
                  <div className="summary">
                    <p>Mode of Payment</p>
                    <p>{transaction.mop}</p>
                  </div>
                  <div className="summary">
                    <p>Sup total</p>
                    <p>
                      {(
                        transaction.total + transaction.discount
                      ).toLocaleString()}
                    </p>
                  </div>
                  <div className="summary">
                    <p>Discount</p>
                    <p>-{transaction.discount.toLocaleString()}</p>
                  </div>
                  <div className="summary">
                    <p>Total</p>
                    <p>{transaction.total.toLocaleString()}</p>
                  </div>
                </div>
                <div className="transactions-con__section__main__details__options">
                  <button
                    onClick={() => {
                      handlePrint();
                    }}
                  >
                    Reprint
                  </button>
                  <button
                    onClick={() => {
                      let isOk = window.confirm(
                        "refund order? this action is irreversible"
                      );
                      // handleBackToTill(transaction);
                      if (isOk && transaction) handleBackToTill(transaction);
                    }}
                  >
                    Back To Till
                  </button>
                </div>
              </>
            )}
          </div>
        </div>
      </section>
    </dialog>
  );
};
