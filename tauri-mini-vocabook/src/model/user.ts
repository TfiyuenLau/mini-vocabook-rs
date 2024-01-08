export interface User {
    email: string;
    pwHash: string;
    userId: number;
    username: string;
    wordbookId: number;
    createTime: string;
    isEffective: number;
}

export interface UserLogin {
    email: string;
    password: string;
}

export interface UserRegister {
    email: string;
    username: string;
    password: string;
    wordbook_id: number;
}
