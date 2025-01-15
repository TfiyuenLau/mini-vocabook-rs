import {User, UserLogin, UserRegister} from "../model/user";
import {ApiResult} from "../model/res";
import {useToast} from "vue-toastification";
import http from "../http.ts";
import {Body} from "@tauri-apps/api/http";

const toast = useToast();

enum UserApiUrl {
    loginUser = "/user/login",
    getUserById = "/user/get_user_by_id",
    registerUser = "/user/register",
    updateUserPassword = "/user/update_user_password",
    updateUser = "/user/update_user",
}

// 登陆校验
const loginUser = async (userLogin: UserLogin): Promise<ApiResult<User>> => {
    const params = {
        email: userLogin.email,
        password: userLogin.password,
    };
    let res = await http(`${UserApiUrl.loginUser}`, {method: "get", params}).catch(_ => {
        toast.error(`服务获取失败`);
    });
    return res?.data as Promise<ApiResult<User>>;
}

// 获取用户信息
const getUserById = async (userId: number): Promise<ApiResult<User>> => {
    const params = {
        user_id: userId,
    };
    let res = await http(`${UserApiUrl.getUserById}`, {method: "get", params}).catch(_ => {
        toast.error(`用户信息获取失败`);
    });
    return res?.data as Promise<ApiResult<User>>;
}

// 用户注册
const registerUser = async (userRegister: UserRegister): Promise<ApiResult<User>> => {
    const body = Body.json(userRegister);
    let res = await http(`${UserApiUrl.registerUser}`, {method: "post", body}).catch(_ => {
        toast.error(`数据提交失败`);
    });
    return res?.data as Promise<ApiResult<User>>;
}

// 更新用户密码
const updateUserPassword = async (email: string, password: string, modifyPw: string): Promise<ApiResult<User>> => {
    let res = await http(`${UserApiUrl.updateUserPassword}?email=${email}&password=${password}&modify_pw=${modifyPw}`, {method: "post"}).catch(_ => {
        toast.error(`数据提交失败`);
    });
    return res?.data as Promise<ApiResult<User>>;
}

// 更新用户信息
const updateUser = async (user: UserRegister): Promise<ApiResult<User>> => {
    const body = Body.json(user)
    let res = await http(`${UserApiUrl.updateUser}`, {method: "post", body}).catch(_ => {
        toast.error(`数据提交失败`);
    });
    return res?.data as Promise<ApiResult<User>>;
}

export {
    loginUser,
    getUserById,
    registerUser,
    updateUserPassword,
    updateUser,
}
