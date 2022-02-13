import { WHITE_PLAYER_ID, BLACK_PLAYER_ID } from './config';
import * as GS from './game-state';
import * as MS from './min-max-tree';

declare const readline: any;

try {
    const playerID: number = parseInt(readline());

    // game loop
    while (true) {
        const gameState: GS.GameState = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        const cardMovesMap: GS.CardMovesMap = {};
        let board: string[] = [];

        for (let i = 0; i < 5; i++) {
            const boardRow: string = readline();
            board.push(boardRow);
        }

        board = board.reverse();
        let wStudentIndex = 0;
        let bStudentIndex = 0;

        board.forEach((row, y) => {
            row.split('').forEach((cell, x) => {
                switch (cell) {
                    case 'W': {
                        GS.initPlayerWizard(gameState, WHITE_PLAYER_ID, x, y);
                        return;
                    }
                    case 'w': {
                        GS.initPlayerStudent(gameState, WHITE_PLAYER_ID, wStudentIndex, x, y);
                        wStudentIndex += 1;
                        return;
                    }
                    case 'B': {
                        GS.initPlayerWizard(gameState, BLACK_PLAYER_ID, x, y);
                        return;
                    }
                    case 'b': {
                        GS.initPlayerStudent(gameState, BLACK_PLAYER_ID, bStudentIndex, x, y);
                        bStudentIndex += 1;
                        return;
                    }
                    default: {
                    }
                }
            });
        });

        let wCardIndex = 0;
        let bCardIndex = 0;

        [0, 1, 2, 3, 4].forEach(() => {
            const inputs: string[] = readline().split(' ');
            const [owner, cardID, dx1, dy1, dx2, dy2, dx3, dy3, dx4, dy4] = inputs.map((input) => parseInt(input));

            GS.initCardMove(cardMovesMap, cardID, dx1, dy1);
            GS.initCardMove(cardMovesMap, cardID, dx2, dy2);
            GS.initCardMove(cardMovesMap, cardID, dx3, dy3);
            GS.initCardMove(cardMovesMap, cardID, dx4, dy4);

            switch (owner) {
                case WHITE_PLAYER_ID: {
                    GS.initPlayerCard(gameState, WHITE_PLAYER_ID, wCardIndex, cardID);
                    wCardIndex += 1;
                    return;
                }
                case BLACK_PLAYER_ID: {
                    GS.initPlayerCard(gameState, BLACK_PLAYER_ID, bCardIndex, cardID);
                    bCardIndex += 1;
                    return;
                }
                case -1: {
                    GS.initMiddleCard(gameState, cardID);
                    return;
                }
                default: {
                }
            }
        });

        const actionCount: number = parseInt(readline());
        for (let i = 0; i < actionCount; i++) {
            var inputs: string[] = readline().split(' ');
            const cardId: number = parseInt(inputs[0]);
            const move: string = inputs[1];
        }

        const rootNode: MS.MinMaxNode = [0, playerID, playerID, 0, '', gameState, []];

        MS.buildMinMaxTree(rootNode, cardMovesMap, 3);

        MS.scoreMinMaxTree(rootNode, 3, -Infinity, +Infinity, true);

        const nextCommand = MS.getNextCommand(rootNode);

        console.log(`${nextCommand}`);
    }
} catch (error_) {
    const error = error_ as Error;
    console.error(error.message);
}
