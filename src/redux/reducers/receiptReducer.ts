import { createSlice } from "@reduxjs/toolkit";
import { PaymentMethod } from "../../pages/auth/LoadEntity";
import { IEntity } from "./entityReducer";
import { ICustomer, Item } from "../../pages/home/till/Till";

export interface IReceipt extends IEntity {
  modeOfPayment: string;
  invoiceNumber: string;
  total: number;
  discount: number;
  customer: string;
  hasSplit: boolean;
  staff: string;
  items: Item[];
}
const RECEIPT = "receipt";
const rawState = localStorage.getItem(RECEIPT);

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
const initialState: IReceipt = rawState
  ? JSON.parse(rawState)
  : {
      _id: "",
      abbrv: "",
      address: "",
      businessName: "",
      categories: "",
      customer: "",
      discount: 0,
      email: "",
      formulations: [],
      hasSplit: false,
      invoiceNumber: "",
      items: [],
      message: "",
      modeOfPayment: "",
      password: "",
      paymentMethods: [],
      phone: "",
      printSilently: false,
      staff: "",
      syncTime: 30000,
      terminal_count: 0,
      terminals: [],
      token: "",
      tokens: [],
      total: 0,
      username: "",
    };

const entityReducer = createSlice({
  name: "receipt",
  initialState,
  reducers: {
    setReceipt: (state, { payload }) => {
      let res = { ...state, ...payload };
      localStorage.setItem(RECEIPT, JSON.stringify(res));
      return res;
    },
    clearReceipt: () => {
      localStorage.removeItem(RECEIPT);
      return initialState;
    },
  },
});

export const { clearReceipt, setReceipt } = entityReducer.actions;

export default entityReducer.reducer;
