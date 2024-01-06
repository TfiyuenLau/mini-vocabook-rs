import axiosHttp from "../axios.http";
import {User, UserLogin, UserRegister} from "../model/user";
import {ApiResult} from "../model/res";
import {useToast} from "vue-toastification";

const toast = useToast();

enum UserApiUrl {
    loginUser = "/api/user/login",
    getUserById = "/api/user/get_user_by_id",
    registerUser = "/api/user/register",
}

// 登陆校验
const loginUser = async (userLogin: UserLogin): Promise<ApiResult<User>> => {
    return await axiosHttp.get(`${UserApiUrl.loginUser}?email=${userLogin.email}&password=${userLogin.password}`).catch(error => {
        toast.error(`服务获取失败：${error.data}`);
    }) as Promise<ApiResult<User>>;
}

// 获取用户信息
const getUserById = async (userId: number): Promise<ApiResult<User>> => {
    return await axiosHttp.get(`${UserApiUrl.getUserById}?user_id=${userId}`).catch(error => {
        toast.error(`用户获取失败：${error.data}`);
    }) as Promise<ApiResult<User>>;
}

// 用户注册
const registerUser = async (userRegister: UserRegister): Promise<ApiResult<User>> => {
    return await axiosHttp.post(`${UserApiUrl.registerUser}`, userRegister).catch(error => {
        toast.error(`数据提交失败：${error.data.status}`);
    }) as Promise<ApiResult<User>>;
}

export {
    loginUser,
    getUserById,
    registerUser,
}
