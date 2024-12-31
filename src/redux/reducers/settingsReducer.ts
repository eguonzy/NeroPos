import { createSlice } from "@reduxjs/toolkit";
import { SETTINGS } from "../../utils/constants";

export interface ISettings {
  showMenu: boolean;
}
const rawState = localStorage.getItem(SETTINGS);

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
const initialState: ISettings | null = rawState
  ? JSON.parse(rawState)
  : { showMenu: false };

const settingsReducer = createSlice({
  name: "settings",
  initialState,
  reducers: {
    setSettings: (_, { payload }) => {
      localStorage.setItem(SETTINGS, JSON.stringify(payload));
      return payload;
    },
    clearSettings: (_) => {
      localStorage.setItem(SETTINGS, JSON.stringify({}));
      return null;
    },
  },
});

export const { clearSettings, setSettings } = settingsReducer.actions;
export default settingsReducer.reducer;
