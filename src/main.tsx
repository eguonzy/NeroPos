import React, {
  createContext,
  Dispatch,
  ReactNode,
  SetStateAction,
  useState,
} from "react";
import ReactDOM from "react-dom/client";

import "./main.scss";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import LoadEntity, { Entity } from "./pages/auth/LoadEntity";
import Login from "./pages/auth/Login";
import Home from "./pages/home/Home";
import Till, { Item } from "./pages/home/till/Till";
import Settings from "./pages/home/settings/Settings";
import Stock from "./pages/home/stock/Stock";
import Transactions from "./pages/home/transactions/Transactions";
import { SnackbarProvider } from "notistack";
import App from "./App";
import { Provider } from "react-redux";
import store from "./redux/store";

const router = createBrowserRouter([
  {
    path: "/load_entity",
    element: <LoadEntity />,
  },
  {
    path: "/",
    element: <Login />,
  },
  {
    path: "/home",
    element: <Home />,
    children: [
      {
        path: "/home",
        element: <Till />,
      },
      {
        path: "/home/stock",
        element: <Stock />,
      },
      {
        path: "/home/transactions",
        element: <Transactions />,
      },
      {
        path: "/home/settings",
        element: <Settings />,
      },
    ],
  },
]);
// interface MyContextType {
//   context: {
//     invoiceNumber?: string;
//     entity?: Entity;
//     showSideBar: boolean;
//     items?: Item[];
//     total?: number;
//     discount?: number;
//     mop?: string;
//     staff?: string;
//     customer?: string;
//   };
//   setContext: Dispatch<
//     SetStateAction<{
//       entity?: Entity;
//       invoiceNumber?: string;
//       showSideBar: boolean;
//       items?: Item[];
//       total?: number;
//       discount?: number;
//       mop?: string;
//       staff?: string;
//       customer?: string;
//     }>
//   >;
// }
// export const AppContext = createContext<MyContextType | undefined>(undefined);
// const MyProvider: React.FC<MyProviderProps> = ({ children }) => {
//   const [context, setContext] = useState<{
//     showSideBar: boolean;
//     items?: Item[];
//     invoiceNumber?: string;
//     total?: number;
//     discount?: number;
//     mop?: string;
//     entity?: Entity;
//     staff?: string;
//     customer?: string;
//   }>({ showSideBar: false });

//   return (
//     <AppContext.Provider value={{ context, setContext }}>
//       {children}
//     </AppContext.Provider>
//   );
// };

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SnackbarProvider maxSnack={50}>
      <Provider store={store}>
        <RouterProvider router={router} />
      </Provider>
    </SnackbarProvider>
  </React.StrictMode>
);

interface MyProviderProps {
  children: ReactNode;
}
