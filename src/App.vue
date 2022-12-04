<script>
import { onMounted } from 'vue';
import { mountChart } from './composable/chart.js';

export default {
  name: 'Chart Demo',
  setup() {
    onMounted(() => {
      const myChart = mountChart(document.getElementById('main'));

      console.log(myChart);

      window.onresize = () => {
        myChart.resize();
      };
    });
  },
};
</script>

<template>
  <div class="container">
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
.container {
  height: 600px;
  width: 600px;
  border-radius: 5px;
}
.chart {
  width: 100%;
  height: 100%;
}
</style>
