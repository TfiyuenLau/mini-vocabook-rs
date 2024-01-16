import {useToast} from "vue-toastification";
import {ApiResult} from "../model/res";
import {LearningWord, MemoryTestsWord, SpellingTestsWord, Word} from "../model/word";
import http from "../http.ts";

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
    const params = {word_id: wordId};
    let res = await http(`${WordApiUrl.getWordById}`, {method: "get", params}).catch(_ => {
        toast.error(`单词信息获取失败，请检查服务器连接。`);
    });
    return res?.data as Promise<ApiResult<Word>>;
}

// 获取指定用户的学习单词集合
const getLearningWords = async (userId: number, wordbookId: number, limit: number): Promise<ApiResult<Array<LearningWord>>> => {
    const params = {
        user_id: userId,
        wordbook_id: wordbookId,
        limit: limit,
    }
    let res = await http(`${WordApiUrl.getLearningWords}`, {method: "get", params}).catch(_ => {
        toast.error(`单词信息获取失败`);
    });
    return res?.data as Promise<ApiResult<Array<LearningWord>>>;
}

// 获取指定用户的复习单词集合
const getReviewWords = async (userId: number, wordbookId: number, limit: number): Promise<ApiResult<Array<LearningWord>>> => {
    const params = {
        user_id: userId,
        wordbook_id: wordbookId,
        limit: limit,
    }
    let res = await http(`${WordApiUrl.getReviewWords}`, {method: "get", params}).catch(_ => {
        toast.error(`单词信息获取失败`);
    });
    return res?.data as Promise<ApiResult<Array<LearningWord>>>;
}

// 获取指定用户的单词记忆测验集合
const getMemoryTestsWords = async (userId: number, limit: number): Promise<ApiResult<Array<MemoryTestsWord>>> => {
    const params = {
        user_id: userId,
        limit: limit,
    }
    let res = await http(`${WordApiUrl.getMemoryTestsWords}`, {method: "get", params}).catch(_ => {
        toast.error(`单词记忆测验数据获取失败`);
    });
    return res?.data as Promise<ApiResult<Array<MemoryTestsWord>>>;
}

// 获取指定用户的拼写测试单词列表
const getSpellingTestsWords = async (userId: number, limit: number): Promise<ApiResult<Array<SpellingTestsWord>>> => {
    const params = {
        user_id: userId,
        limit: limit,
    }
    let res = await http(`${WordApiUrl.getSpellingTestsWords}`, {method: "get", params}).catch(_ => {
        toast.error(`拼写测验单词数据获取失败`);
    });
    return res?.data as Promise<ApiResult<Array<SpellingTestsWord>>>;
}

export {
    getWordById,
    getLearningWords,
    getReviewWords,
    getMemoryTestsWords,
    getSpellingTestsWords,
}
