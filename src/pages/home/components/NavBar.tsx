import React, { ChangeEvent, useContext } from "react";
import { IProduct } from "../till/Till";
import { DateTime } from "luxon";
import { useAppDispatch, useAppSelector } from "../../../redux/hooks";
import { setSettings } from "../../../redux/reducers/settingsReducer";

function NavBar({
  onChange,
  products,
  onClick,
  searchRef,
}: {
  onChange: (e: ChangeEvent<HTMLInputElement>) => Promise<void>;
  onClick: (product: IProduct) => void;
  products: IProduct[];
  searchRef: React.RefObject<HTMLInputElement>;
}) {
  const dispatch = useAppDispatch();
  const settings = useAppSelector((state) => state.settings);
  return (
    <div className="nav-bar">
      <div
        onClick={() => dispatch(setSettings({ showMenu: !settings?.showMenu }))}
        className={"ham " + (settings?.showMenu ? "open" : "")}
      >
        <div></div>
        <div></div>
        <div></div>
      </div>
      <h3>mPOS</h3>
      <form>
        <input
          ref={searchRef}
          onChange={(e) => onChange(e)}
          type="search"
          autoComplete="off"
          name="query"
          placeholder="Search products..."
          autoFocus={true}
        />
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
          className="feather feather-search"
        >
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
        {products.length > 0 && (
          <div className="search-result-con">
            {products?.map((product) => {
              let now = DateTime.now();
              let sixMonths = now.plus({ months: 6 });
              let year = now.plus({ months: 12 });
              let expiry_date = DateTime.fromISO(product.expiry_date);
              return (
                <div
                  className={
                    now >= expiry_date
                      ? "danger"
                      : sixMonths >= expiry_date
                      ? "warning"
                      : year >= expiry_date
                      ? "caution"
                      : ""
                  }
                  key={product._id}
                  onClick={(e) => onClick(product)}
                >
                  <p>{product.name}</p>
                  <p>&#8358;{product.sell_price.toLocaleString()}</p>
                </div>
              );
            })}
          </div>
        )}
      </form>
    </div>
  );
}

export default NavBar;
