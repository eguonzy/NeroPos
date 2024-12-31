import { createSlice } from "@reduxjs/toolkit";
import { PaymentMethod } from "../../pages/auth/LoadEntity";

export interface IEntity {
  _id: string;
  username: string;
  address: string;
  email: string;
  phone: string;
  message: string;
  password: string;
  formulations: string[];
  businessName: string;
  tokens: { token: string }[];
  terminal_count: number;
  terminals: { terminal: string }[];
  printSilently: boolean;
  syncTime: number;
  categories: string[];
  token: string;
  abbrv: string;
  paymentMethods: PaymentMethod[];
  terminal: string;
}
const ENTITY = "entity";
const rawState = localStorage.getItem(ENTITY);

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
const initialState: IEntity | null = rawState ? JSON.parse(rawState) : null;

const entityReducer = createSlice({
  name: "entity",
  initialState,
  reducers: {
    setEntity: (_, { payload }) => {
      localStorage.setItem(ENTITY, JSON.stringify(payload));
      return payload;
    },
    clearEntity: () => {
      localStorage.removeItem(ENTITY);
      return null;
    },
  },
});

export const { setEntity, clearEntity } = entityReducer.actions;

export default entityReducer.reducer;
