import axios from 'axios';
import config from '../config';
import { Dispenser } from '../models/dispenser';
import { Drink, Ingredient } from '../models/drink';

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

export const updateDispenser = async (
  dispenser: Dispenser
): Promise<Dispenser> => {
  const { data } = await api.post('/dispenser', dispenser);
  return data;
};

export const makeDrink = async (ingredients: Ingredient[]) => {
  const { data } = await api.post('/drinks/make', ingredients);
  return data;
};
