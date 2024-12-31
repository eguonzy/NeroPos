import { configureStore } from "@reduxjs/toolkit";
import entity from "./reducers/entityReducer";
import receipt from "./reducers/receiptReducer";
import user from "./reducers/staffReducer";
import settings from "./reducers/settingsReducer";

const store = configureStore({
  reducer: { entity, receipt, user, settings },
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch;

export default store;
