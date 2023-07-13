
export type Fields = string[][];
export interface Position2D {
  x: number;
  y: number;
}

export interface FieldStatus {
  fields: Fields;
  lastStonePosition?: Position2D;
}

export const createNewFields = (ySize: number, xSize: number): string[][] => {
  const result: string[][] = [];
  for (let i = 0; i < ySize; i++) {
    const yArray: Array<"" | "O" | "X"> = [];
    for (let j = 0; j < xSize; j++) {
      yArray.push("");
    }
    result.push(yArray);
  }
  return result;
};

export interface OmPlayer {
  getDescription(): { nickname: string; tactics: string; };

  dropTheStone(
    fieldStatus: FieldStatus,
    yourFlag: "O" | "X",
  ): Promise<Position2D>;
}

export class Runner implements OmPlayer {
  getDescription(): { nickname: string; tactics: string; } {
    return {
      nickname: 'innfi',
      tactics: 'observing'
    };
  }

  async dropTheStone(
    fieldStatus: FieldStatus,
    yourFlag: "O" | "X",
  ): Promise<Position2D> {
    return {
      x: 1,
      y: 1,
    };
  }

  findConditionHorizontal(fieldStatus: FieldStatus, targetFlag: 'O'|'X'): Position2D | undefined {
    const fields = fieldStatus.fields;
    const lastPos = fieldStatus.lastStonePosition!;

    let posX = lastPos.x;
    while (posX > 0 && fields[posX][lastPos.y] === targetFlag) {
      posX--;
    }

    if (posX < 0) return undefined;

    return {
      x: posX,
      y: lastPos.y,
    };
  }
}