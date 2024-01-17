import {useToast} from "vue-toastification";
import {LearningRecord, UploadRecord} from "../model/learning_record";
import {ApiResult} from "../model/res";
import http from "../http.ts";
import {Body} from "@tauri-apps/api/http";

const toast = useToast();

enum LearningRecordApiUrl {
    get14daysRecordStatistic = "/record/get_14days_record_statistic",
    getMasteryLevelStatistic = "/record/get_mastery_level_statistic",
    getWordCloudStatistic = "/record/get_word_cloud_statistic",
    getDateCheckInStatistic = "/record/get_date_check_in_statistic",
    insertOrUpdateRecord = "/record/insert_or_update_record",
}

// 获取用户近十四日统计信息
const get14daysRecordStatistic = async (userId: number): Promise<ApiResult<Array<any>>> => {
    const params = {
        user_id: userId,
    };
    let res = await http(`${LearningRecordApiUrl.get14daysRecordStatistic}`, {method: "get", params}).catch(_ => {
        toast.error("近十四日学习统计数据获取失败");
    });
    return res?.data as Promise<ApiResult<Array<any>>>;
}

// 获取用户的单词本熟练程度统计信息
const getMasteryLevelStatistic = async (userId: number, wordbookId: number): Promise<ApiResult<Array<any>>> => {
    const params = {
        user_id: userId,
        wordbook_id: wordbookId,
    };
    let res = await http(`${LearningRecordApiUrl.getMasteryLevelStatistic}`, {method: "get", params}).catch(_ => {
        toast.error("单词本熟练程度统计数据获取失败");
    });
    return res?.data as Promise<ApiResult<Array<any>>>;
}

// 获取用户的词云统计数据
const getWordCloudStatistic = async (userId: number): Promise<ApiResult<Array<any>>> => {
    const params = {
        user_id: userId,
    };
    let res = await http(`${LearningRecordApiUrl.getWordCloudStatistic}`, {method: "get", params}).catch(_ => {
        toast.error("用户词云统计数据获取失败");
    });
    return res?.data as Promise<ApiResult<Array<any>>>;
}

// 获取用户的每日打卡记录统计数据
const getDateCheckInStatistic = async (userId: number, limit: number): Promise<ApiResult<Array<any>>> => {
    const params = {
        user_id: userId,
        limit: limit,
    };
    let res = await http(`${LearningRecordApiUrl.getDateCheckInStatistic}`, {method: "get", params}).catch(_ => {
        toast.error("单词本熟练程度统计数据获取失败");
    });
    return res?.data as Promise<ApiResult<Array<any>>>;
}

// 更新学习记录熟练度
const insertOrUpdateRecord = async (data: UploadRecord): Promise<ApiResult<LearningRecord>> => {
    const body = Body.json(data);
    let res = await http(`${LearningRecordApiUrl.insertOrUpdateRecord}`, {method: "post", body}).catch(_ => {
        toast.error("学习记录更新失败");
    });
    return res?.data as Promise<ApiResult<LearningRecord>>;
}

export {
    get14daysRecordStatistic,
    getMasteryLevelStatistic,
    getWordCloudStatistic,
    getDateCheckInStatistic,
    insertOrUpdateRecord,
}
