import {Box} from '@chakra-ui/react';
import {invoke} from '@tauri-apps/api';
import {useEffect, useState} from 'react';
import {AreaChart, CartesianGrid, Area, Tooltip, XAxis, YAxis} from 'recharts';

export default function Home() {
  const [data, setData] = useState([]);
  useEffect(() => {
    invoke('charge_history', {productId: 123}).then(data => {
      setData(data.filter(d => d.percentage != 0));
    }).catch(console.error);
  }, []);
  return (
    <Box p={5}>
      Chart:
      <AreaChart width={600} height={400} data={data} margin={{top: 5, right: 20, bottom: 5, left: 0}}>
        <defs>
          <linearGradient id="colorUv" x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor="#8884d8" stopOpacity={0.8} />
            <stop offset="95%" stopColor="#8884d8" stopOpacity={0} />
          </linearGradient>
        </defs>
        <Area type="monotone" dataKey="percentage" stroke="#8884d8" />
        <CartesianGrid stroke="#444" strokeDasharray="5 5" />
        <XAxis dataKey="created_at" />
        <YAxis />
        <Tooltip />
      </AreaChart>
    </Box>
  );
}
