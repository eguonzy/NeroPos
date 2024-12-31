import React, { useContext, useEffect } from "react";
import "./home.scss";
import NavBar from "./components/NavBar";
import SideBar from "./components/SideBar";
import HomeWindow from "./components/HomeWindow";
import { DateTime } from "luxon";
import { useAppSelector } from "../../redux/hooks";
function Home() {
  const receipt = useAppSelector((state) => state.receipt);
  const entity = useAppSelector((state) => state.entity);
  useEffect(() => {
    let day = `${DateTime.now().year}-${DateTime.now().month}-${
      DateTime.now().day
    }`;
    let raw = localStorage.getItem("count");
    if (!raw || JSON.parse(raw).date !== day) {
      let trxCount = {
        count: 0,
        date: day,
      };
      localStorage.setItem("count", JSON.stringify(trxCount));
    }
  }, []);
  return (
    <>
      <div className="home-con">
        <div className="home-body">
          <SideBar />
          <HomeWindow />
        </div>
      </div>

      <div className="receipt">
        <p className="business-name">{receipt.businessName}</p>
        <p className="business-address">{receipt.address}</p>
        <p className="business-address">{receipt.phone}</p>

        <div className="detail">
          <p>30-12-24</p>
          <p>09:40pm</p>
        </div>
        <div className="detail">
          <p>Payment Method</p>
          <p>{receipt.modeOfPayment}</p>
        </div>
        <div className="detail">
          <p>Invoice</p>
          <p>{receipt.invoiceNumber}</p>
        </div>
        <div className="detail">
          <p>Staff</p>
          <p>{receipt.staff}</p>
        </div>
        {receipt.customer && (
          <div className="detail">
            <p>Customer</p>
            <p>{receipt.customer}</p>
          </div>
        )}

        <div className="line"></div>

        {receipt.items?.map((item) => {
          return (
            <div key={item.product_id} className="invoice-con">
              <p className="name">{item.name}</p>
              <p>
                {item.quantity.toLocaleString()} x{" "}
                {item.sell_price.toLocaleString()}{" "}
                {item.discount > 0 ? "(-\u20a6" + item.discount + ")" : ""} =
                &#8358;
                {(item.total - item.discount).toLocaleString()}
              </p>
            </div>
          );
        })}

        <div className="line"></div>
        <div className="detail">
          <p>#items</p>
          <p>{receipt.items?.length}</p>
        </div>
        <div className="detail">
          <p>Subtotal</p>
          <p>
            &#8358;
            {(receipt?.discount ?? 0 + (receipt?.total ?? 0)).toLocaleString()}
          </p>
        </div>
        <div className="detail">
          <p>Discount</p>
          <p>&#8358;{receipt.discount?.toLocaleString()}</p>
        </div>
        <div className="detail">
          <p>Total</p>
          <p>&#8358;{receipt.total}</p>
        </div>
        <div className="line"></div>
        <div className="line"></div>
        <p className="footer">{entity.message}</p>
      </div>
    </>
  );
}

export default Home;
