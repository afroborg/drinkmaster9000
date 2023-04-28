import { derived, writable } from 'svelte/store';
import { Drink, Ingredient } from '../models/drink';
import toast from 'svelte-french-toast';
import { makeDrink } from './api';
import type { Dispenser } from '../models/dispenser';

const ML_PER_MSECOND = 100;
const ROTATION_TIME_PER_MSECOND = 30;
const WAIT_TIME = 2000;
const FILL_MINI_DISPENSER_TIME_MSECOND = 3000;
const MAX_DISPENSES_PER_PUSH = 35;
const MS_PER_DEGREE = 50;

export const useDispenser = (drinks: Drink[], dispenser: Dispenser) => {
  const {
    pour_speed_ml_ms = ML_PER_MSECOND,
    refill_delay_ms = FILL_MINI_DISPENSER_TIME_MSECOND,
    rotation_delay_ms = ROTATION_TIME_PER_MSECOND,
    angle_between = 0,
  } = dispenser ?? {};

  const isPouring = writable<boolean>(false);

  const dispenserAmount = writable<number[]>(
    drinks.map(() => Math.round(100 / drinks.length ?? 1))
  );

  const glassAmount = writable<number>(330);

  const fromPercentToMl = derived(glassAmount, ($glassAmount) => {
    return (percent: number) => {
      return Math.round($glassAmount * (percent / 100));
    };
  });

  const timeForFills = derived(
    [dispenserAmount, fromPercentToMl],
    ([$dispenserAmount, $fromPercentToMl]) => {
      if ($dispenserAmount.length <= 0) {
        return 0;
      }

      return $dispenserAmount.reduce((prev, curr) => {
        if (curr <= 0) {
          return prev;
        }

        const amount = $fromPercentToMl(curr);

        const timesToFill = Math.floor(amount / MAX_DISPENSES_PER_PUSH);

        const time = amount * pour_speed_ml_ms + timesToFill * refill_delay_ms;

        return prev + time;
      }, 0);
    }
  );

  const timeForRotations = derived(dispenserAmount, ($dispenserAmount) => {
    if ($dispenserAmount.length <= 0) {
      return 0;
    }

    const [time, lastIndex] = $dispenserAmount.reduce(
      ([time, currIndex], amount, index) => {
        if (amount <= 0) {
          return [time, currIndex];
        }

        const indexesToMove = Math.abs(currIndex - index);

        const tTime =
          angle_between * indexesToMove * MS_PER_DEGREE + rotation_delay_ms * 2;

        return [time + tTime, index];
      },
      [0, 0]
    );

    const backRotation = lastIndex * MS_PER_DEGREE + rotation_delay_ms * 2;

    return time + backRotation;
  });

  const timeForWaiting = derived(dispenserAmount, ($dispenserAmount) => {
    if ($dispenserAmount.length <= 0) {
      return 0;
    }

    return $dispenserAmount.reduce((time, amount) => {
      if (amount <= 0) {
        return time;
      }

      return time + WAIT_TIME;
    });
  });

  const totalTime = derived(
    [timeForFills, timeForRotations, timeForWaiting],
    ([$timeForFills, $timeForRotations, $timeForWaiting]) => {
      return $timeForFills + $timeForRotations + $timeForWaiting;
    }
  );

  const createDrink = derived<
    [typeof dispenserAmount, typeof fromPercentToMl],
    () => Promise<void>
  >(
    [dispenserAmount, fromPercentToMl],
    ([$dispenserAmount, $fromPercentToMl]) => {
      return async () => {
        const ingredients = $dispenserAmount
          .map<Ingredient>((amount, index) => {
            if (amount <= 0) {
              return null;
            }

            const amountMl = $fromPercentToMl(amount);

            return {
              amount: amountMl,
              index,
            };
          })
          .filter(Boolean);

        if (!ingredients.length) {
          toast.error('Inga ingredienser valda');
          return;
        }

        try {
          const res = makeDrink(ingredients);
          isPouring.set(true);
          toast.success('Drinken påbörjas');
          await res;

          toast.success('Drinken är nu klar');
        } catch {
          toast.error('Kunde inte göra drinken');
        }

        isPouring.set(false);
      };
    }
  );

  const reset = () => {};

  return {
    isPouring,
    dispenserAmount,
    glassAmount,
    totalTime,
    createDrink,
    reset,
  };
};
