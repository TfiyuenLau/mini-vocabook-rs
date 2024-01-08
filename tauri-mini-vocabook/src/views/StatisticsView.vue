<script setup lang="ts">
import * as echarts from 'echarts';
import 'echarts-wordcloud';
import {onMounted, ref} from "vue";

const dateBarChartRef = ref();
const masteryPieChartRef = ref();
const wordcloudChartRef = ref();

// 打卡日期柱状图
const initDateBarChart = () => {
  const barChart = echarts.init(dateBarChartRef.value);

  const chartData = [
    {date: '2024-01-01', value: 30},
    {date: '2024-01-02', value: 50},
    {date: '2024-01-03', value: 80},
    {date: '2024-01-04', value: 60},
    {date: '2024-01-05', value: 40},
    {date: '2024-01-06', value: 70},
    {date: '2024-01-07', value: 90},
    {date: '2024-01-08', value: 30},
    {date: '2024-01-09', value: 50},
    {date: '2024-01-10', value: 80},
    {date: '2024-01-11', value: 60},
    {date: '2024-01-12', value: 40},
    {date: '2024-01-13', value: 70},
    {date: '2024-01-14', value: 90},
  ];
  const dates = chartData.map(item => item.date);
  const values = chartData.map(item => item.value);

  const option = {
    title: {
      text: "近14日打卡日期柱状统计图",
      left: "center",
      top: "8px",
    },
    xAxis: {
      type: "category",
      data: dates,
    },
    yAxis: {
      type: "value"
    },
    series: [
      {
        data: values,
        type: 'bar',
        label: {
          show: true,
          position: 'top',
        },
      },
    ],
  };
  barChart.setOption(option);
}

// 熟练程度饼状图
const initMasteryPieChart = () => {
  const chart = echarts.init(masteryPieChartRef.value);

  const data = [
    {name: '陌生', value: 10},
    {name: '不熟悉', value: 20},
    {name: '熟悉', value: 30},
    {name: '熟练', value: 40},
  ];

  const option = {
    title: {
      text: "单词本熟练程度",
      left: "center",
      top: "8px",
    },
    series: [
      {
        name: "掌握程度",
        type: 'pie',
        radius: ['40%', '60%'], // 内外半径，实现圆环图
        data: data,
      },
    ],
    roseType: 'area',
  };

  chart.setOption(option);
}

// 单词词云图
const initWordcloudChart = () => {
  const chart = echarts.init(wordcloudChartRef.value);

  const data = [
    {name: 'vue', value: 30},
    {name: 'tauri', value: 15},
    {name: 'axum', value: 35},
    {name: 'sea', value: 60},
    {name: 'orm', value: 20},
    {name: 'java', value: 30},
    {name: 'springboot', value: 10},
  ];

  const option = {
    title: {
      text: "记忆词云图",
      left: "center",
      top: "8px",
    },
    series: [{
      type: "wordCloud",
      /**
       * 绘制词云的形状, 值为回调函数 或 关键字, 默认 circle
       *  关键字:
       *
       * circle（圆形）  词的数量不太多的时候，效果不明显，它会趋向于画一个椭圆
       * cardioid（苹果形或心形曲线）
       * diamond（菱形 正方形）
       * triangle-forward（三角形-向前）
       * triangle（三角形-直立）
       * pentagon（五边形）
       * star（星形）
       */
      shape: "circle",
      // 保持 maskImage 的纵横比或形状的纵横比为 1：1
      keepAspect: true,
      /**
       * 词云轮廓图，支持为 HTMLImageElement, HTMLCanvasElement，不支持路径字符串, 不包含白色区域; 可选选项
       * shape选项将随着云的形状增长而继续应用
       * 有形状限制的时候，最好用背景图来实现，而且这个背景图一定要放base64的，不然词云画不出来
       */
      // maskImage: maskImage,

      left: 'center',
      top: 'center',
      width: '100%',
      height: '100%',
      right: null,
      bottom: null,
      // 词云文本大小范围,  默认为最小12像素，最大60像素
      sizeRange: [16, 56],
      // 词云文字旋转范围和步长。 文本将通过旋转在[-90，90]范围内随机旋转步骤45
      // 如果都设置为 0 , 则是水平显示
      rotationRange: [-90, 90],
      rotationStep: 45,
      /**
       * 词间距, 距离越大，单词之间的间距越大, 单位像素
       * 这里间距太小的话，会出现大词把小词套住的情况，比如一个大的口字，中间会有比较大的空隙，这时候他会把一些很小的字放在口字里面，这样的话，鼠标就无法选中里面的那个小字
       */
      gridSize: 8,
      // 设置为true可以使单词部分在画布之外绘制, 允许绘制大于画布大小的单词
      drawOutOfBound: false,
      /**
       * 布局的时候是否有动画
       * 注意：禁用时，当单词较多时，将导致UI阻塞。
       */
      layoutAnimation: true,
      // 这是全局的文字样式，相对应的还可以对每个词设置字体样式
      textStyle: {
        fontFamily: 'sans-serif',
        fontWeight: 'bold',
        // 颜色可以用一个函数来返回字符串
        color: function () {
          // 随机颜色
          return (
              'rgb(' +
              [
                Math.round(Math.random() * 160),
                Math.round(Math.random() * 160),
                Math.round(Math.random() * 160),
              ].join(',') +
              ')'
          )
        },
      },
      // 鼠标hover的特效样式
      emphasis: {
        focus: 'self',
        textStyle: {
          textShadowBlur: 10,
          textShadowColor: '#999'
        }
      },

      /**
       * 词云数据，必须是一个数组，每个数组项必须有name和value属性
       * 设置单个文本的样式：  textStyle
       *
       * 例：{
       name: '',
       value: 40,
       textStyle: {
       }
       },
       */
      data: data,
    }]
  };

  chart.setOption(option);
}

onMounted(() => {
  initDateBarChart();
  initMasteryPieChart();
  initWordcloudChart();
})

</script>

<template>
  <v-container>
    <v-card image="/image/material-pattern-wallpaper.jpg">
      <v-row justify="start">
        <v-col :cols="10" :offset="1" class="mt-4 mb-2">
          <v-card>
            <div ref="dateBarChartRef" style="width: 100%;height: 256px;"></div>
          </v-card>
        </v-col>
      </v-row>
      <v-row justify="start" class="mt-2 mb-4">
        <v-col :cols="5" :offset="1">
          <v-card>
            <div ref="masteryPieChartRef" style="width: 100%;height: 256px;"></div>
          </v-card>
        </v-col>
        <v-col :cols="5">
          <v-card>
            <div ref="wordcloudChartRef" style="width: 100%;height: 256px;"></div>
          </v-card>
        </v-col>
      </v-row>
    </v-card>
  </v-container>
</template>

<style scoped>

</style>