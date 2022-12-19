import * as echarts from 'echarts';
import { convertFileSrc } from '@tauri-apps/api/tauri';

export function mountChart(element) {
  const myChart = echarts.init(element);
  const dataURL = convertFileSrc('localhost', 'plot');

  fetch(dataURL).then(r => r.arrayBuffer()).then(data =>{
    data =  Array.from(new Float32Array(data));

    const option = {
      tooltip: {
        trigger: 'axis',
        position: function (pt) {
          return pt;
        }
      },
      title: {
        left: 'center',
        text: 'Large Area Chart'
      },
      toolbox: {
        feature: {
          dataZoom: {
            yAxisIndex: 'none'
          },
        }
      },
      xAxis: {
        type: 'category',
      },
      yAxis: {
        type: 'value',
      },
      dataZoom: [
        {
          type: 'inside',
          start: 0,
          end: 100,
        },
        {
          start: 0,
          end: 100
        }
      ],
      series: [
        {
          name: 'Fake Data',
          type: 'line',
          symbol: 'none',
          sampling: 'lttb',
          lineStyle: {
            color: 'rgb(255, 70, 131)',
            width: 1
          },
          data: data
        }
      ]
    };

    myChart.setOption(option);
  });

  return myChart;
}

export function updateChart(myChart) {
  const dataURL = convertFileSrc('localhost', 'plot');

  fetch(dataURL).then(r => r.arrayBuffer()).then(data =>{
    if (data.byteLength != 0) {
      data =  Array.from(new Float32Array(data));

      const option = {
        series: [
          {
            data: data
          }
        ]
      };

      myChart.setOption(option);
    }
  });

  return myChart;
}
