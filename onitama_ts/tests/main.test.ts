import { expect } from 'chai';
import { describe, it } from 'mocha';

import * as GS from '../src/game-state';
import * as MS from '../src/min-max-tree';

describe('main', () => {
    it('does something', () => {
        // Given

        // When

        // Then
        const playerID = 0;

        const gameState: GS.GameState = [
            4, 1, 2, 8, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1,
        ];

        const rootNode: MS.MinMaxNode = [0, playerID, playerID, 0, '', gameState, []];

        const cardMovesMap: GS.CardMovesMap = {
            '1': [-6, 5, -4],
            '4': [-1, 5, 1],
            '5': [-6, -1, 1, 6],
            '13': [-4, 1, -6, -1],
            '15': [5, -10],
        };

        MS.buildMinMaxTree(rootNode, cardMovesMap, 1);

        expect(rootNode).to.deep.equal([
            0,
            0,
            0,
            0,
            '',
            [4, 1, 2, 8, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
            [
                [
                    1,
                    0,
                    1,
                    0,
                    '4 C1C2',
                    [128, 1, 2, 8, 16, 5, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 4, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '1 C1C2',
                    [128, 1, 2, 8, 16, 4, 1, 5, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 1, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '4 A1A2',
                    [4, 32, 2, 8, 16, 5, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 4, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '1 A1A2',
                    [4, 32, 2, 8, 16, 4, 1, 5, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 1, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '4 B1B2',
                    [4, 1, 64, 8, 16, 5, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 4, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '1 B1B2',
                    [4, 1, 64, 8, 16, 4, 1, 5, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 1, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '4 D1D2',
                    [4, 1, 2, 256, 16, 5, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 4, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '1 D1D2',
                    [4, 1, 2, 256, 16, 4, 1, 5, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 1, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '4 E1E2',
                    [4, 1, 2, 8, 512, 5, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 4, -1],
                    [],
                ],
                [
                    1,
                    0,
                    1,
                    0,
                    '1 E1E2',
                    [4, 1, 2, 8, 512, 4, 1, 5, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 1, -1],
                    [],
                ],
            ],
        ]);
    });

    it('does something', () => {
        // Given

        // When

        // Then
        const gameState: GS.GameState = [
            4, 1, 2, 32768, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1,
        ];

        const cardMovesMap: GS.CardMovesMap = {
            '1': [-6, 5, -4],
            '4': [-1, 5, 1],
            '5': [-6, -1, 1, 6],
            '13': [-4, 1, -6, -1],
            '15': [5, -10],
        };

        const clonedGameState = GS.cloneGameState(gameState);

        GS.applyPlayerMoveToGameState(clonedGameState, cardMovesMap, 1, 2, 0, 2);

        expect(clonedGameState).to.deep.equal([
            4, 1, 2, 0, 16, 4, 1, 1, 1, 4194304, 1048576, 32768, 8388608, 16777216, 5, 1, 15, 1, 13, -1,
        ]);
    });

    it('test', () => {
        const tests = [
            {
                state: [4, 1, 2, 8, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 0,
                score: 0,
            },
            {
                state: [4, 1, 2, 8, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 1,
                score: 0,
            },

            {
                state: [4, 1, 2, 8, 16, 4, 1, 1, 1, 0, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 0,
                score: 1,
            },
            {
                state: [0, 1, 2, 8, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 1,
                score: 1,
            },
            {
                state: [0, 1, 2, 8, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 0,
                score: -1,
            },
            {
                state: [4, 0, 2, 8, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 0,
                score: -0.2,
            },
            {
                state: [4, 0, 0, 0, 0, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 0,
                score: -0.8,
            },
            {
                state: [4, 0, 0, 0, 0, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 1,
                score: 0.8,
            },
            {
                state: [4194304, 0, 0, 0, 0, 4, 1, 1, 1, 12, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 0,
                score: 1,
            },
            {
                state: [12, 0, 0, 0, 0, 4, 1, 1, 1, 4, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1],
                maximizingPlayerID: 0,
                score: -1,
            },
        ];

        tests.forEach((test) => {
            expect(GS.getGameStateScore(test.state as GS.GameState, test.maximizingPlayerID)).to.deep.equal(test.score);
        });
    });

    it('minMax', () => {
        const playerID = 0;

        const gameState: GS.GameState = [
            4, 1, 2, 8, 16, 4, 1, 1, 1, 4194304, 1048576, 2097152, 8388608, 16777216, 13, 1, 15, 1, 5, 1,
        ];

        const rootNode: MS.MinMaxNode = [0, playerID, playerID, 0, '', gameState, []];

        const cardMovesMap: GS.CardMovesMap = {
            '1': [-6, 5, -4],
            '4': [-1, 5, 1],
            '5': [-6, -1, 1, 6],
            '13': [-4, 1, -6, -1],
            '15': [5, -10],
        };
        const start = new Date().getTime();
        MS.buildMinMaxTree(rootNode, cardMovesMap, 4);
        console.log(new Date().getTime() - start);
        MS.scoreMinMaxTree(rootNode, 4, -Infinity, +Infinity, true);
        console.log(new Date().getTime() - start);
        const nextCommand = MS.getNextCommand(rootNode);
        console.log(new Date().getTime() - start);
        console.log(nextCommand);
    });
});
