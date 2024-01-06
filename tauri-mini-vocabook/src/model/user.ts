export interface User {
    createTime: string;
    email: string;
    isEffective: number;
    pwHash: string;
    userId: number;
    username: string;
    wordbookId: number;
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
