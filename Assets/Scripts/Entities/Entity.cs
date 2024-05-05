using System;
using System.Collections;
using System.Collections.Generic;
using ObjectExtensions;
using UnityEngine;
using UnityEngine.Serialization;

public class Entity : MonoBehaviour
{
    [Header( "Data" )]
    [SerializeField] public EntityData entityData;

    [SerializeField] public PoolSO health;

    public void FixedUpdate()
    {
        if ( entityData ) entityData.position = transform.position;
    }
}
public enum StatType
{
    Focus,
    Mind,
    Body
}
