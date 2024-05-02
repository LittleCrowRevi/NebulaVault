using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Serialization;

[CreateAssetMenu(menuName = "Variables/Int Variable")]
public class IntVariable : ScriptableObject, ISerializationCallbackReceiver
{
    public int value;

    [NonSerialized] public int RuntimeValue;

    public void OnBeforeSerialize()
    {
    }

    public void OnAfterDeserialize()
    {
        RuntimeValue = value;
    }
}