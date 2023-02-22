export type Drink = {
  name: string;
  ingredients: Ingredient[];
};

export type Ingredient = {
  name: string;
  volume: number;
};

export type DrinkResponse = {
  drinks: Drink[];
};
