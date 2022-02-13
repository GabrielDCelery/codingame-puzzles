import { GameState } from './game-state';
import * as GS from './game-state';
import { NUM_OF_CARDS_PER_PLAYER, NUM_OF_PIECES_PER_PLAYER } from './config';

export enum MinMaxIndex {
    Depth,
    RootPlayerID,
    CurrentPlayerID,
    Score,
    Command,
    GameState,
    ChildNodes,
}

type Depth = number;
type RootPlayerID = number;
type CurrentPlayerID = number;
type ChildMinMaxNodes = MinMaxNode[];
type Score = number;
type Command = string;

export type MinMaxNode = [Depth, RootPlayerID, CurrentPlayerID, Score, Command, GameState, ChildMinMaxNodes];

export const buildMinMaxTree = (minMaxNode: MinMaxNode, cardMovesMap: GS.CardMovesMap, targetDepth: number) => {
    if (minMaxNode[MinMaxIndex.Depth] === targetDepth || GS.isGameFinished(minMaxNode[MinMaxIndex.GameState])) {
        return;
    }

    [0, 1, 2, 3, 4].forEach((_, pieceIndex) => {
        const doesPlayerPieceExist = GS.doesPlayerPieceExist(
            minMaxNode[MinMaxIndex.GameState],
            minMaxNode[MinMaxIndex.CurrentPlayerID],
            pieceIndex
        );

        if (!doesPlayerPieceExist) {
            return;
        }

        [0, 1].forEach((_, cardIndex) => {
            const cardID = GS.getPlayerCardID(
                minMaxNode[MinMaxIndex.GameState],
                minMaxNode[MinMaxIndex.CurrentPlayerID],
                cardIndex
            );

            cardMovesMap[cardID].forEach((_, moveIndex) => {
                const isPlayerMoveValid = GS.isPlayerMoveValid(
                    minMaxNode[MinMaxIndex.GameState],
                    cardMovesMap,
                    minMaxNode[MinMaxIndex.CurrentPlayerID],
                    pieceIndex,
                    cardIndex,
                    moveIndex
                );

                if (!isPlayerMoveValid) {
                    return;
                }

                const piecePositionOnBoardBeforeMove = GS.getPiecePositionOnBoard(
                    minMaxNode[MinMaxIndex.GameState],
                    minMaxNode[MinMaxIndex.CurrentPlayerID],
                    pieceIndex
                );

                const clonedGameState = GS.cloneGameState(minMaxNode[MinMaxIndex.GameState]);

                GS.applyPlayerMoveToGameState(
                    clonedGameState,
                    cardMovesMap,
                    minMaxNode[MinMaxIndex.CurrentPlayerID],
                    pieceIndex,
                    cardIndex,
                    moveIndex
                );

                const piecePositionOnBoardAfterMove = GS.getPiecePositionOnBoard(
                    clonedGameState,
                    minMaxNode[MinMaxIndex.CurrentPlayerID],
                    pieceIndex
                );

                const newNode: MinMaxNode = [
                    minMaxNode[MinMaxIndex.Depth] + 1,
                    minMaxNode[MinMaxIndex.RootPlayerID],
                    GS.getOpponentID(minMaxNode[MinMaxIndex.CurrentPlayerID]),
                    0,
                    `${cardID} ${piecePositionOnBoardBeforeMove}${piecePositionOnBoardAfterMove}`,
                    clonedGameState,
                    [],
                ];

                minMaxNode[MinMaxIndex.ChildNodes].push(newNode);

                buildMinMaxTree(newNode, cardMovesMap, targetDepth);
            });
        });
    });
};

export const scoreMinMaxTree = (
    minMaxNode: MinMaxNode,
    depth: number,
    alpha: number,
    beta: number,
    isMinMaxingPlayer: boolean
): number => {
    if (depth === 0 || GS.isGameFinished(minMaxNode[MinMaxIndex.GameState])) {
        const score = GS.getGameStateScore(minMaxNode[MinMaxIndex.GameState], minMaxNode[MinMaxIndex.RootPlayerID]);
        minMaxNode[MinMaxIndex.Score] = score;
        return score;
    }

    if (isMinMaxingPlayer) {
        let maxEval = -Infinity;
        let maxAlpha = alpha;
        for (let i = 0, iMax = minMaxNode[MinMaxIndex.ChildNodes].length; i < iMax; i++) {
            const childNode = minMaxNode[MinMaxIndex.ChildNodes][i];
            let nodeEval = scoreMinMaxTree(childNode, depth - 1, maxAlpha, beta, false);
            maxEval = Math.max(maxEval, nodeEval);
            maxAlpha = Math.max(maxAlpha, nodeEval);
            if (beta <= maxAlpha) {
                break;
            }
        }
        minMaxNode[MinMaxIndex.Score] = maxEval;
        return maxEval;
    }

    let minEval = +Infinity;
    let minBeta = beta;
    for (let i = 0, iMax = minMaxNode[MinMaxIndex.ChildNodes].length; i < iMax; i++) {
        const childNode = minMaxNode[MinMaxIndex.ChildNodes][i];
        let nodeEval = scoreMinMaxTree(childNode, depth - 1, alpha, minBeta, true);
        minEval = Math.min(minEval, nodeEval);
        minBeta = Math.min(minBeta, nodeEval);
        if (minBeta <= alpha) {
            break;
        }
    }
    minMaxNode[MinMaxIndex.Score] = minEval;
    return minEval;
};

export const getNextCommand = (minMaxNode: MinMaxNode): string => {
    let maxScore = -Infinity;
    let nextCommand = '';
    minMaxNode[MinMaxIndex.ChildNodes].forEach((childNode) => {
        if (childNode[MinMaxIndex.Score] >= maxScore) {
            maxScore = childNode[MinMaxIndex.Score];
            nextCommand = childNode[MinMaxIndex.Command];
        }
    });
    return nextCommand;
};
