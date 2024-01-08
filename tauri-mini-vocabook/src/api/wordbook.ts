import {useToast} from "vue-toastification";
import {ApiResult} from "../model/res";
import axiosHttp from "../axios.http";
import {Word} from "../model/word";
import {Wordbook} from "../model/wordbook.ts";

const toast = useToast();

enum WordbookApiUrl {
    getWordbookById = "/api/wordbook/get_wordbook_by_id",
    getWordsByWordbookId = "/api/wordbook/get_words_by_wordbook_id",
    getWordbookWordCount = "/api/wordbook/get_wordbook_word_count",
    getWordbookProgress = "/api/wordbook/get_wordbook_progress",
}

// 通过wordbook_id获取对应的单词本
const getWordbookById = async (wordbookId: number): Promise<ApiResult<Wordbook>> => {
    return await axiosHttp.get(`${WordbookApiUrl.getWordbookById}?wordbook_id=${wordbookId}`).catch(_ => {
        toast.error(`目标单词本获取失败`);
    });
}

// 通过wordbook_id获取对应的单词分页列表
const getWordsByWordbookId = async (wordbookId: number, offset: number): Promise<ApiResult<Array<Word>>> => {
    return await axiosHttp.get(`${WordbookApiUrl.getWordsByWordbookId}?wordbook_id=${wordbookId}&offset=${offset}`).catch(_ => {
        toast.error(`分页单词列表获取失败`);
    });
}

const getWordbookWordCount = async (wordbookId: number): Promise<ApiResult<number>> => {
    return await axiosHttp.get(`${WordbookApiUrl.getWordbookWordCount}?wordbook_id=${wordbookId}`).catch(_ => {
        toast.error(`分页单词列表获取失败`);
    });
}

const getWordbookProgress = async (userId: number, wordbookId: number): Promise<ApiResult<number>> => {
    return await axiosHttp.get(`${WordbookApiUrl.getWordbookProgress}?user_id=${userId}&wordbook_id=${wordbookId}`).catch(_ => {
        toast.error(`分页单词列表获取失败`);
    });
}

export {
    getWordbookById,
    getWordsByWordbookId,
    getWordbookWordCount,
    getWordbookProgress,
}
