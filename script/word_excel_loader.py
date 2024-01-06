import pandas as pd
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
from sqlalchemy import create_engine
from mini_vocabook_orm import Word, WordWordbookMapping

# 数据库连接配置
db_url = "mysql://root:123456@localhost:3306/mini_vocabook"
engine = create_engine(db_url, echo=True)
Base = declarative_base()
Base.metadata.create_all(engine)  # 创建表结构

# 创建会话并读取xlsx文件
Session = sessionmaker(bind=engine)
session = Session()
df = pd.read_excel('./excel/考研考纲词汇5500.xlsx')
df = df.where(df.notnull(), None)

# 插入item到数据库
for index, row in df.iterrows():
    word = Word(word_id=index + 1 + 10012, word=row['单词'].strip(), phonogram=row['音标'], definition=row['释义'])
    mapping = WordWordbookMapping(word_id=index + 1 + 10012, wordbook_id=3)
    # print(word.word_id, word.word, word.phonogram, word.definition, mapping.wordbook_id)
    if word.word is None:
        continue
    session.add(word)
    session.add(mapping)

# 提交事务
session.commit()
print("Successfully!")
