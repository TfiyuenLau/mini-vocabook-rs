export interface Word {
    wordId: number;
    word: string;
    phonogram: string;
    definition: string;
    exampleSentence: string;
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

export interface SpellingTestsWord {
    wordId: number;
    word: string;
    phonogram: string;
    definition: string;
    quiz: SpellingQuiz;
    input: string;
    status: "right" | "error" | "undefined";
}

export type SpellingQuiz = {
    head: string;
    middle: string;
    tail: string;
};
