export type Servo = {
  pin: number;
  max_us: number;
  min_us: number;
  neutral_us: number;
  start_angle: number;
  end_angle: number;
  is_reversed: boolean;
};
