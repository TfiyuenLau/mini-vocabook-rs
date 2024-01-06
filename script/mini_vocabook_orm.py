from sqlalchemy import Column, String, Text, BigInteger, Integer, DateTime, ForeignKey, Boolean
from sqlalchemy.ext.declarative import declarative_base
from datetime import datetime

Base = declarative_base()


class Word(Base):
    __tablename__ = 'word'
    word_id = Column(BigInteger, primary_key=True, comment='单词ID')
    word = Column(String(64), nullable=False, comment='单词')
    phonogram = Column(String(128), nullable=False, comment='单词音标')
    definition = Column(Text, comment='单词释义')
    example_sentence = Column(Text, comment='单词示例语句')


class Wordbook(Base):
    __tablename__ = 'wordbook'
    wordbook_id = Column(BigInteger, primary_key=True, comment='单词本ID')
    book_name = Column(String(32), nullable=False, comment='单词本名称')
    introduction = Column(Text, comment='单词本介绍')


class WordWordbookMapping(Base):
    __tablename__ = 'word_wordbook_mapping'
    word_id = Column(BigInteger, ForeignKey('word.word_id'), nullable=False, comment='单词ID外键')
    wordbook_id = Column(BigInteger, ForeignKey('wordbook.wordbook_id'), nullable=False, comment='单词本ID外键',
                         primary_key=True)


class User(Base):
    __tablename__ = 'user'
    user_id = Column(BigInteger, primary_key=True, comment='用户ID')
    email = Column(String(64), unique=True, nullable=False, comment='用户邮箱')
    username = Column(String(32), unique=True, nullable=False, comment='用户昵称')
    pw_hash = Column(String(128), nullable=False, comment='密码')
    wordbook_id = Column(BigInteger, ForeignKey('wordbook.wordbook_id'), nullable=False, comment='选择的单词本')
    create_time = Column(DateTime, nullable=False, default=datetime.now, comment='用户创建时间')
    is_effective = Column(Boolean, nullable=False, default=True, comment='是否有效')


class LearningRecord(Base):
    __tablename__ = 'learning_record'
    record_id = Column(BigInteger, primary_key=True, comment='学习记录ID')
    user_id = Column(BigInteger, ForeignKey('user.user_id'), nullable=False, comment='用户ID外键')
    word_id = Column(BigInteger, ForeignKey('word.word_id'), nullable=False, comment='单词ID外键')
    mastery_level = Column(Integer, nullable=False, comment='熟练程度')
    learning_dt = Column(DateTime, nullable=False, default=datetime.now, comment='学习记录日期时间')
    is_effective = Column(Boolean, nullable=False, default=True, comment='是否有效')
