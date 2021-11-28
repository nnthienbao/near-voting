import React, { useState, useEffect } from "react";
import Highcharts from "highcharts";
import HighchartsReact from "highcharts-react-official";

export default function Chart({ data }) {
  const [chartOptions, setChartOptions] = useState({});
  useEffect(() => {
    const options = {
      chart: {
        type: "spline",
      },
      tooltip: {
        shared: true,
      },
      plotOptions: {
        series: {
          marker: {
            enabled: true,
          },
        },
      },
      title: {
        text: "Election progress",
      },
      xAxis: {
        type: "datetime",
      },
      yAxis: {
        title: {
          text: "Total votes",
        },
      },
      series: data,
    };
    setChartOptions(options);
  }, [data]);
  return (
    <HighchartsReact
      highcharts={Highcharts}
      options={chartOptions}
      oneToOne={true}
    />
  );
}
