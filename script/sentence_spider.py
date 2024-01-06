import requests
from lxml import etree
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

from script.mini_vocabook_orm import Word

Base = declarative_base()

# 数据库连接字符串，根据你的实际情况修改
db_url = "mysql://root:123456@localhost:3306/mini_vocabook"
engine = create_engine(db_url)  # echo=True表示打印SQL语句
Session = sessionmaker(bind=engine)
session = Session()

headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 '
                  'Safari/537.36'
}


def fetch_word_data(word):
    # 海词词典:https://corp.dict.cn/
    url = f'https://corp.dict.cn/search?q={word}'
    response = requests.get(url, headers=headers)

    if response.status_code == 200:
        # 使用XPath提取相关数据
        tree = etree.HTML(response.text)
        example_sentence = tree.xpath('//*[@id="content"]/div[1]/div[3]/div[1]/ol/li/text()')

        if len(example_sentence) % 2 == 0:
            # 重新组织成对的中英文例句
            paired_example_sentences = [(example_sentence[i], example_sentence[i + 1]) for i in
                                        range(0, len(example_sentence), 2)]

            # 将每对中英文例句连接成字符串
            joined_example_sentences = ['\n'.join(pair) for pair in paired_example_sentences]
            result_string = '\n'.join(joined_example_sentences)

            return {
                'word': word,
                'example_sentence': result_string if result_string else None
            }

    return None


def update_word_in_database(word_data):
    words_to_update = session.query(Word).filter(Word.word == word_data['word']).all()

    if words_to_update:
        for existing_word in words_to_update:
            existing_word.example_sentence = word_data['example_sentence']
        session.commit()
        print(f"Successfully updated data for word: {word_data['word']}")
    else:
        print(f"Word {word_data['word']} not found in the database.")


def main():
    words_from_database = session.query(Word.word).all()
    words_to_crawl = [word[0] for word in words_from_database]

    for word in words_to_crawl:
        existing_words = session.query(Word).filter(Word.word == word).all()

        # 判断是否所有条目都有例句
        if all(existing_word.example_sentence for existing_word in existing_words):
            print(f"Example sentences already exist for all entries with word: {word}")
            continue

        word_data = fetch_word_data(word)
        if word_data:
            update_word_in_database(word_data)


if __name__ == "__main__":
    main()
