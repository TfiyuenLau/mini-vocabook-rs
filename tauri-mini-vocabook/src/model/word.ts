export interface Word {
    definition: string;
    exampleSentence: string;
    phonogram: string;
    word: string;
    wordId: number;
}

export interface MemoryTestsWord {
    wordId: number;
    word: string;
    phonogram: string;
    definition: string;
    option: string[];
    selected: number[];
    isCorrect: boolean;
}
