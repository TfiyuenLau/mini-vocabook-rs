import {useToast} from "vue-toastification";
import axiosHttp from "../axios.http";
import {LearningRecord, UploadRecord} from "../model/learning_record";
import {ApiResult} from "../model/res";

const toast = useToast();

enum LearningRecordApiUrl {
    get14daysRecordStatistic = "/api/record/get_14days_record_statistic",
    getMasteryLevelStatistic = "/api/record/get_mastery_level_statistic",
    getWordCloudStatistic = "/api/record/get_word_cloud_statistic",
    getDateCheckInStatistic = "/api/record/get_date_check_in_statistic",
    insertOrUpdateRecord = "/api/record/insert_or_update_record",
}

// 获取用户近十四日统计信息
const get14daysRecordStatistic = async (userId: number): Promise<ApiResult<Array<any>>> => {
    return await axiosHttp.get(`${LearningRecordApiUrl.get14daysRecordStatistic}?user_id=${userId}`).catch(_ => {
        toast.error("近十四日学习统计数据获取失败");
    });
}

// 获取用户的单词本熟练程度统计信息
const getMasteryLevelStatistic = async (userId: number, wordbookId: number): Promise<ApiResult<Array<any>>> => {
    return await axiosHttp.get(`${LearningRecordApiUrl.getMasteryLevelStatistic}?user_id=${userId}&wordbook_id=${wordbookId}`).catch(_ => {
        toast.error("单词本熟练程度统计数据获取失败");
    });
}

// 获取用户的词云统计数据
const getWordCloudStatistic = async (userId: number): Promise<ApiResult<Array<any>>> => {
    return await axiosHttp.get(`${LearningRecordApiUrl.getWordCloudStatistic}?user_id=${userId}`).catch(_ => {
        toast.error("用户词云统计数据获取失败");
    });
}

// 获取用户的每日打卡记录统计数据
const getDateCheckInStatistic = async (userId: number, limit: number): Promise<ApiResult<Array<any>>> => {
    return await axiosHttp.get(`${LearningRecordApiUrl.getDateCheckInStatistic}?user_id=${userId}&limit=${limit}`).catch(_ => {
        toast.error("单词本熟练程度统计数据获取失败");
    });
}

// 更新学习记录熟练度
const insertOrUpdateRecord = async (data: UploadRecord): Promise<ApiResult<LearningRecord>> => {
    return await axiosHttp.post(`${LearningRecordApiUrl.insertOrUpdateRecord}`, data).catch(_ => {
        toast.error("学习记录更新失败");
    });
}

export {
    get14daysRecordStatistic,
    getMasteryLevelStatistic,
    getWordCloudStatistic,
    getDateCheckInStatistic,
    insertOrUpdateRecord,
}
