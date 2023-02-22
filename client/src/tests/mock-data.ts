import { Drink } from '../models/api';

export const MOJITO: Drink = {
  name: 'Mojito',
  ingredients: [
    {
      name: 'Rum',
      volume: 50,
    },
    {
      name: 'Lime',
      volume: 20,
    },
    {
      name: 'Sugar',
      volume: 10,
    },
    {
      name: 'Mint',
      volume: 10,
    },
    {
      name: 'Soda',
      volume: 10,
    },
  ],
};

export const NEGRONI: Drink = {
  name: 'Negroni',
  ingredients: [
    {
      name: 'Gin',
      volume: 30,
    },
    {
      name: 'Campari',
      volume: 30,
    },
    {
      name: 'Sweet Vermouth',
      volume: 30,
    },
  ],
};

export const VODKA_SODA: Drink = {
  name: 'Vodka Soda',
  ingredients: [
    {
      name: 'Vodka',
      volume: 50,
    },
    {
      name: 'Soda',
      volume: 50,
    },
  ],
};

export const MOCK_DRINKS: Drink[] = [MOJITO, NEGRONI];
