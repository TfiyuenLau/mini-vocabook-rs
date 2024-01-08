import {useToast} from "vue-toastification";
import axiosHttp from "../axios.http";
import {LearningRecord, UploadRecord} from "../model/learning_record";
import {ApiResult} from "../model/res";

const toast = useToast();

enum LearningRecordApiUrl {
    insertOrUpdateRecord = "/api/record/insert_or_update_record",
}

// 更新学习记录熟练度
const insertOrUpdateRecord = async (data: UploadRecord): Promise<ApiResult<LearningRecord>> => {
    return await axiosHttp.post(`${LearningRecordApiUrl.insertOrUpdateRecord}`, data).catch(_ => {
        toast.error("学习记录更新失败");
    });
}

export {
    insertOrUpdateRecord,
}
