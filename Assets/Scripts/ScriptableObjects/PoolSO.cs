using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Serialization;

[CreateAssetMenu(menuName = "Variables/Pool")]
public class PoolSO : ScriptableObject
{
    public int baseValue;
    public int currentValue;
}
