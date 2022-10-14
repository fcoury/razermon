import { Box } from '@chakra-ui/react';
import { invoke } from '@tauri-apps/api';
import { DateTime } from 'luxon';
import { useEffect, useState } from 'react';
import {
  Area,
  AreaChart,
  CartesianGrid,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';

interface BatteryData {
  product_id: number;
  created_at: string | number;
  percentage: number;
  charging: boolean;
}

interface CustomTooltipProps {
  active?: boolean;
  payload: any;
}

const CustomTooltip = ({ active, payload: data }: CustomTooltipProps) => {
  const { payload } = data?.[0] || {};
  if (active && payload) {
    let dateTime = DateTime.fromMillis(payload.millis).toFormat('MM/dd HH:mm');
    return (
      <div
        style={{ background: '#eee', opacity: 0.8, color: '#222', padding: 5 }}
      >
        {dateTime} - {payload.percentage}%
      </div>
    );
  }

  return null;
};

export default function Home() {
  const [productId, setProductId] = useState(null);
  const [status, setStatus] = useState(null);
  const [data, setData] = useState<BatteryData[]>([]);
  const [batteryStats, setBatteryStats] = useState({});

  useEffect(() => {
    invoke('selected_product_id')
      .then((productId: any) => {
        setProductId(productId);
      })
      .catch(console.error);
  }, []);

  useEffect(() => {
    if (!productId) return;

    invoke('device_status', { productId })
      .then((status: any) => {
        setStatus(status);
      })
      .catch(console.error);

    invoke<BatteryData[]>('charge_history', { productId })
      .then((res: BatteryData[]) => {
        let data = res.filter((d) => d.percentage > 0);
        let values = data
          .filter((d, i) => i == 0 || d.percentage != data[i - 1].percentage)
          .map((d) => ({
            ...d,
            millis: DateTime.fromFormat(
              d.created_at as string,
              'yyyy-MM-dd HH:mm:ss',
              { zone: 'utc' },
            ).toMillis(),
          }));
        setData(values);
      })
      .catch(console.error);

    invoke('battery_stats', { productId })
      .then(([status, remaining]: any) => {
        console.log('status/remaining', status, remaining);
        setBatteryStats({ status, remaining });
      })
      .catch(console.error);
  }, [productId]);

  if (!data.length) return null;

  return (
    <Box p={5}>
      <div>{(status as any)?.name}</div>
      <div>{(batteryStats as any)?.remaining}</div>
      <AreaChart width={900} height={400} data={data}>
        <defs>
          <linearGradient id="colorUv" x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor="#8884d8" stopOpacity={0.8} />
            <stop offset="95%" stopColor="#8884d8" stopOpacity={0} />
          </linearGradient>
        </defs>
        <Area type="monotone" dataKey="percentage" stroke="#8884d8" />
        <CartesianGrid stroke="#444" strokeDasharray="5 5" />
        <XAxis
          dataKey="millis"
          name="Time"
          type="number"
          domain={['auto', 'auto']}
          tickFormatter={(value) => {
            const date = DateTime.fromMillis(value as number);
            return date.toFormat('MM/dd HH:mm');
          }}
        />
        <YAxis />
        <Tooltip
          content={(props) => (
            <CustomTooltip active={props.active} payload={props.payload} />
          )}
        />
      </AreaChart>
    </Box>
  );
}
