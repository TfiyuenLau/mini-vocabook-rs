import {useToast} from "vue-toastification";
import {ApiResult} from "../model/res";
import axiosHttp from "../axios.http";
import {MemoryTestsWord, SpellingTestsWord, Word} from "../model/word";

const toast = useToast();

enum WordApiUrl {
    getWordById = "/api/word/get_word_by_id",
    getLearningWords = "/api/word/get_learning_words",
    getReviewWords = "/api/word/get_review_words",
    getMemoryTestsWords = "/api/word/get_memory_tests_words",
    getSpellingTestsWords = "/api/word/get_spelling_tests_words",
}

// 通过wordbook_id获取对应的单词分页列表
const getWordById = async (wordId: number): Promise<ApiResult<Word>> => {
    return await axiosHttp.get(`${WordApiUrl.getWordById}?word_id=${wordId}`).catch(_ => {
        toast.error(`单词信息获取失败，请检查服务器连接。`);
    });
}

// 获取指定用户的学习单词集合
const getLearningWords = async (userId: number, wordbookId: number, limit: number): Promise<ApiResult<Array<Word>>> => {
    return await axiosHttp.get(`${WordApiUrl.getLearningWords}?user_id=${userId}&wordbook_id=${wordbookId}&limit=${limit}`)
        .catch(_ => {
            toast.error(`单词信息获取失败`);
        });
}

// 获取指定用户的复习单词集合
const getReviewWords = async (userId: number, wordbookId: number, limit: number): Promise<ApiResult<Array<Word>>> => {
    return await axiosHttp.get(`${WordApiUrl.getReviewWords}?user_id=${userId}&wordbook_id=${wordbookId}&limit=${limit}`)
        .catch(_ => {
            toast.error(`单词信息获取失败`);
        });
}

// 获取指定用户的单词记忆测验集合
const getMemoryTestsWords = async (userId: number, limit: number): Promise<ApiResult<Array<MemoryTestsWord>>> => {
    return await axiosHttp.get(`${WordApiUrl.getMemoryTestsWords}?user_id=${userId}&limit=${limit}`)
        .catch(_ => toast.error(`单词记忆测验数据获取失败`));
}

// 获取指定用户的拼写测试单词列表
const getSpellingTestsWords = async (userId: number, limit: number): Promise<ApiResult<Array<SpellingTestsWord>>> => {
    return await axiosHttp.get(`${WordApiUrl.getSpellingTestsWords}?user_id=${userId}&limit=${limit}`)
        .catch(_ => {
            toast.error(`拼写测验单词数据获取失败`);
        });
}

export {
    getWordById,
    getLearningWords,
    getReviewWords,
    getMemoryTestsWords,
    getSpellingTestsWords,
}
