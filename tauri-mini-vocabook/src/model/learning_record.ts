export interface LearningRecord {
    recordId: number;
    userId: number;
    wordId: number;
    masteryLevel: number;
    createTime: string;
    updateTime: string;
    isEffective: number;
}

export interface UploadRecord {
    userId: number;
    wordId: number;
    flag: boolean;
}

