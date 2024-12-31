import React, { FormEvent, useContext, useState } from "react";
import "./auth.scss";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
import { useAppDispatch } from "../../redux/hooks";
import { setEntity } from "../../redux/reducers/entityReducer";
import Loader from "../../utils/Loader";

export interface Entity {
  _id: string;
  username: string;
  businessName: string;
  email: string;
  phone: string;
  message: string;
  password: string;
  address: string;
  terminal: string;
  createdAt: string;
  updatedAt: string;
  terminal_count: number;
  __v: number;
  printSilently: number;
  syncTime: number;
  paymentMethods: PaymentMethod[];
}
export interface PaymentMethod {
  name: string;
  hidden: boolean;
  _id: string;
  value: number;
  isActive?: boolean;
}
function LoadEntity() {
  const navigate = useNavigate();
  let [username, setUsername] = useState("");
  const dispatch = useAppDispatch();
  let [loading, setLoading] = useState(false);
  let [password, setPassword] = useState("");
  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      let res = await invoke("load_entity", { username, password });

      dispatch(setEntity(res));
      navigate("/");
      console.log(res);
    } catch (error) {
      console.log(error);
    }
    setLoading(false);
  };
  return (
    <div className="load-entity-con">
      {loading ? (
        <div className="loader-c">
          {" "}
          <Loader />
        </div>
      ) : (
        <div className="load-entity-con__form">
          <h3>Load Entity</h3>
          <form onSubmit={handleSubmit} className="form">
            <div className="input-con">
              <label htmlFor="">Username</label>
              <input
                onChange={(e) => setUsername(e.target.value)}
                type="text"
                value={username}
              />
            </div>
            <div className="input-con">
              <label htmlFor="">Password</label>
              <input
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                type="password"
              />
            </div>
            <button disabled={loading}>Login</button>
          </form>
        </div>
      )}
    </div>
  );
}

export default LoadEntity;
