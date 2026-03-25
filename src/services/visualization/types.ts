export interface DataPoint {
  timestamp: number;
  value: number;
  label?: string;
}

export interface ChartConfig {
  type: 'line' | 'bar' | 'area' | 'candlestick';
  title: string;
  dataKey: string;
  color?: string;
  yAxisLabel?: string;
  xAxisLabel?: string;
}

export interface DashboardWidget {
  id: string;
  title: string;
  chartConfig: ChartConfig;
  data: DataPoint[];
  position?: { x: number; y: number };
  size?: { width: number; height: number };
}

export interface DashboardLayout {
  id: string;
  name: string;
  widgets: DashboardWidget[];
  createdAt: number;
}

export interface VisualizationAnalytics {
  id: string;
  widgetId: string;
  action: 'zoom' | 'pan' | 'export' | 'interact';
  timestamp: number;
  duration?: number;
}

export interface WebSocketMessage {
  type: 'price' | 'transaction' | 'balance';
  data: Record<string, unknown>;
  timestamp: number;
}
