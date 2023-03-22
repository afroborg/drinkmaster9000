import axios from 'axios';
import config from '../config';
import { Dispenser } from '../models/dispenser';
import { Drink } from '../models/drink';

const api = axios.create({
  baseURL: config.api_url,
});

export const getDrinks = async (): Promise<Drink[]> => {
  const { data } = await api.get('/drinks');
  return data;
};

export const getDispenser = async (): Promise<Dispenser> => {
  const { data } = await api.get('/dispenser');
  return data;
};
