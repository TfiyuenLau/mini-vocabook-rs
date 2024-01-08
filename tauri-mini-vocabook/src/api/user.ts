import axiosHttp from "../axios.http";
import {User, UserLogin, UserRegister} from "../model/user";
import {ApiResult} from "../model/res";
import {useToast} from "vue-toastification";

const toast = useToast();

enum UserApiUrl {
    loginUser = "/api/user/login",
    getUserById = "/api/user/get_user_by_id",
    registerUser = "/api/user/register",
    updateUser = "/api/user/update_user",
}

// 登陆校验
const loginUser = async (userLogin: UserLogin): Promise<ApiResult<User>> => {
    return await axiosHttp.get(`${UserApiUrl.loginUser}?email=${userLogin.email}&password=${userLogin.password}`).catch(_ => {
        toast.error(`服务获取失败`);
    });
}

// 获取用户信息
const getUserById = async (userId: number): Promise<ApiResult<User>> => {
    return await axiosHttp.get(`${UserApiUrl.getUserById}?user_id=${userId}`).catch(_ => {
        toast.error(`用户获取失败`);
    });
}

// 用户注册
const registerUser = async (userRegister: UserRegister): Promise<ApiResult<User>> => {
    return await axiosHttp.post(`${UserApiUrl.registerUser}`, userRegister).catch(_ => {
        toast.error(`数据提交失败`);
    });
}

// 更新用户信息
const updateUser = async (user: UserRegister): Promise<ApiResult<User>> => {
    return await axiosHttp.post(`${UserApiUrl.updateUser}`, user).catch(_ => {
        toast.error(`用户信息修改失败，请检查密码准确性`);
    });
}

export {
    loginUser,
    getUserById,
    registerUser,
    updateUser,
}
