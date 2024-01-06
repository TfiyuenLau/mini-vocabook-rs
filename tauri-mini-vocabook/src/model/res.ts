export interface ApiResult<T> {
    data: T;
    message: string;
    status: number;
}