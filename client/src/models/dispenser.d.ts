import { Servo } from './servo';

export type Dispenser = {
  current_index: number;
  angle_between: number;
  rotation_delay_ms: number;
  pour_speed_ml_ms: number;
  cup_rotator: Servo;
  pusher: Servo[];
};
