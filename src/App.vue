<script>
import { onMounted } from 'vue';
import { mountChart, updateChart } from './composable/chart.js';

export default {
  name: 'Chart Demo',
  setup() {
    onMounted(() => {
      const myChart = mountChart(document.getElementById('main'));

      window.onresize = () => {
        myChart.resize();
      };

      // TODO Button to control frequency
      window.setInterval(() => {
        updateChart(myChart);
      }, 1000);
    });
  },
};
</script>

<template>
  <div class="chart-container">
    <div id="main" class="chart"></div>
  </div>
</template>

<style>
body {
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}
#app {
  height: 100%;
  width: 100%;
  padding: 20px;
}
.chart-container {
  height: 600px;
  width: 600px;
  border-radius: 5px;
}
.chart {
  width: 100%;
  height: 100%;
}
</style>
