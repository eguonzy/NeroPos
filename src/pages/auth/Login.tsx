import React, { FormEvent, useContext, useEffect, useState } from "react";
import "./auth.scss";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
import { STAFF_KEY } from "../../utils/constants";
import { useAppDispatch } from "../../redux/hooks";
import { setUser } from "../../redux/reducers/staffReducer";
import { useSnackbar } from "notistack";

export interface Staff {
  staffId: number;
  username: string;
  name: string;
  surname: string;
  position: string;
}
function Login() {
  let navigate = useNavigate();
  const dispatch = useAppDispatch();
  let [username, setUsername] = useState("admin");
  let [loading, setLoading] = useState(false);
  const { enqueueSnackbar: notify } = useSnackbar();
  let [password, setPassword] = useState("admin");
  let checkEntity = async () => {
    let hasEntity: boolean = await invoke("check_entity");
    if (!hasEntity) navigate("/load_entity");
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      let req: Staff = await invoke("login", { username, password });

      if (req.name) {
        dispatch(setUser(req));
        setLoading(false);
        navigate("/home");
      }
    } catch (error) {
      notify("Invalid username and password combination", {
        variant: "warning",
      });
      console.log(error);
      setLoading(false);
      if (error instanceof Error) notify(error.message, { variant: "error" });
    }
  };
  useEffect(() => {
    checkEntity();
  }, []);
  return (
    <div className="auth-con">
      <div className="auth-con__login-form">
        <h3>Welcome Back</h3>

        <form onSubmit={handleSubmit} className="login-form">
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
              onChange={(e) => setPassword(e.target.value)}
              type="password"
              value={password}
            />
          </div>
          <button disabled={loading}>Login</button>
        </form>
      </div>
    </div>
  );
}

export default Login;
