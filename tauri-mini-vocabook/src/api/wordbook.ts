import {useToast} from "vue-toastification";
import {ApiResult} from "../model/res";
import {Word} from "../model/word";
import {Wordbook} from "../model/wordbook.ts";
import http from "../http.ts";

const toast = useToast();

enum WordbookApiUrl {
    getWordbookById = "/wordbook/get_wordbook_by_id",
    getWordsByWordbookId = "/wordbook/get_words_by_wordbook_id",
    getWordbookWordCount = "/wordbook/get_wordbook_word_count",
    getWordbookProgress = "/wordbook/get_wordbook_progress",
}

// 通过wordbook_id获取对应的单词本
const getWordbookById = async (wordbookId: number): Promise<ApiResult<Wordbook>> => {
    const params = {
        wordbook_id: wordbookId,
    }
    let res = await http(`${WordbookApiUrl.getWordbookById}`, {method: "get", params}).catch(_ => {
        toast.error(`目标单词本获取失败`);
    });
    return res?.data as Promise<ApiResult<Wordbook>>;
}

// 通过wordbook_id获取对应的单词分页列表
const getWordsByWordbookId = async (wordbookId: number, offset: number): Promise<ApiResult<Array<Word>>> => {
    const params = {
        wordbook_id: wordbookId,
        offset: offset,
    }
    let res = await http(`${WordbookApiUrl.getWordsByWordbookId}`, {method: "get", params}).catch(_ => {
        toast.error(`分页单词列表获取失败`);
    });
    return res?.data as Promise<ApiResult<Array<Word>>>;
}

// 获取单词本中单词数
const getWordbookWordCount = async (wordbookId: number): Promise<ApiResult<number>> => {
    const params = {
        wordbook_id: wordbookId,
    }
    let res = await http(`${WordbookApiUrl.getWordbookWordCount}`, {method: "get", params}).catch(_ => {
        toast.error(`单词数获取失败`);
    });
    return res?.data as Promise<ApiResult<number>>;
}

// 获取单词本进度数据
const getWordbookProgress = async (userId: number, wordbookId: number): Promise<ApiResult<number>> => {
    const params = {
        user_id: userId,
        wordbook_id: wordbookId,
    }
    let res = await http(`${WordbookApiUrl.getWordbookProgress}`, {method: "get", params}).catch(_ => {
        toast.error(`单词本进度数据获取失败`);
    });
    return res?.data as Promise<ApiResult<number>>;
}

export {
    getWordbookById,
    getWordsByWordbookId,
    getWordbookWordCount,
    getWordbookProgress,
}
