import {fetch} from '@tauri-apps/plugin-http';
import qs from 'qs';

// 检查 URL 是否为绝对路径
const isAbsoluteURL = (url: string): boolean => {
    return /^([a-z][a-z\d+-.]*:)?\/\/+/i.test(url);
};

// 合并 baseURL 和 relativeURL 以构建完整的 URL
const combineURLs = (baseURL: string, relativeURL: string): string => {
    return relativeURL
        ? baseURL.replace(/\/+$/, '') + '/' + relativeURL.replace(/^\/+/, '')
        : baseURL;
};

// 构建完整的请求路径
const buildFullPath = (baseURL: string, requestedURL: string) => {
    if (baseURL && !isAbsoluteURL(requestedURL)) {
        return combineURLs(baseURL, requestedURL);
    }
    return requestedURL;
};

// 在 URL 上添加查询参数
const buildURL = (url: string, params: any): string => {
    if (!params) {
        return url;
    }
    const serializedParams = qs.stringify(params);
    if (serializedParams) {
        url += (url.indexOf('?') === -1 ? '?' : '&') + serializedParams;
    }
    return url;
};

// 服务器地址
const server = 'http://localhost:3030'; // 上线时，将 http://localhost:3030 替换为真实后端服务地址
const baseURL = `${server}/api`;

// 请求体类型常量
const BODY_TYPE = {
    Form: 'Form',
    Json: 'Json',
    Text: 'Text',
    Bytes: 'Bytes',
};

enum ResponseType {
    JSON = 1,
    Text = 2,
    Binary = 3
}

// 公共请求选项
const commonOptions = {
    timeout: 60,
    responseType: ResponseType.JSON,
};

// 发起 HTTP 请求的主要函数
const http = async (url: string, options: any = {}) => {
    const params = {...options.params};
    if (!options.headers)
        options.headers = {};
    // TODO: 在 headers 中添加 token 或 cookie 等信息

    if (options?.body) {
        if (options.type === BODY_TYPE.Form) {
            options.headers["Content-Type"] = "application/json";
        }
    }

    options = {...commonOptions, ...options};

    try {
        let request = new Request(buildURL(buildFullPath(baseURL, url), params), {
            method: options.method,
            headers: options.headers,
            body: options.body,
        });
        const response = await fetch(request);
        const data = await response.json();
        if (response.status >= 200 && response.status < 400) {
            return {data};
        }
        return Promise.reject({status: response.status, data});
    } catch (err) {
        console.error(err);
        return Promise.reject(err);
    }
};

export default http;