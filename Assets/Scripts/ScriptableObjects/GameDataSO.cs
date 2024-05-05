
using UnityEngine;

[CreateAssetMenu(menuName = "Data/Game Data")]
public class GameDataSO : ScriptableObject
{
    public string LastActiveOverworldScene;
    public Vector2 PlayerPosition;
}