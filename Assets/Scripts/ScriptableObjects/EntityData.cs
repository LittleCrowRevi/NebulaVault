using UnityEngine;
using UnityEngine.Serialization;

[CreateAssetMenu( menuName = "Variables/Entity Data" )]
public class EntityData : ScriptableObject
{
    public Vector2 position;

    public string id;
    public int    baseHealth;
    public int    focus;
    public int    mind;
    public int    body;
}