import { Drink } from '../models/api';
import { MOCK_DRINKS } from '../tests/mock-data';

const API_URL = 'http://localhost:3000/api';

export const getDrinks = async (): Promise<Drink[]> => {
  return Promise.resolve(MOCK_DRINKS);
  // const response = await fetch(`${API_URL}/drinks`);
  // return response.json();
};

export const getDrink = async (name: string): Promise<Drink> => {
  return Promise.resolve(MOCK_DRINKS[0]);
  // const response = await fetch(`${API_URL}/drinks/${name}`);
  // return response.json();
};
