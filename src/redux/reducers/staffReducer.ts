import { createSlice } from "@reduxjs/toolkit";
import { STAFF_KEY } from "../../utils/constants";

export interface IStaff {
  staffId: number;
  username: string;
  name: string;
  surname: string;
  position: string;
}
const rawState = localStorage.getItem(STAFF_KEY);

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
const initialState: IStaff | null = rawState ? JSON.parse(rawState) : null;

const userReducer = createSlice({
  name: "user",
  initialState,
  reducers: {
    setUser: (_, { payload }) => {
      localStorage.setItem(STAFF_KEY, JSON.stringify(payload));
      return payload;
    },
    clearUser: (_) => {
      localStorage.setItem(STAFF_KEY, JSON.stringify({}));
      return null;
    },
  },
});

export const { clearUser, setUser } = userReducer.actions;
export default userReducer.reducer;
