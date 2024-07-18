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

    [SerializeField] public int Health;
    [SerializeField] public int CurrentHealth;

    public int Focus;
    public int Body;
    public int Mind;

    [Header( "Broadcasting Events" )]
    public IntEventChannelSO m_healthChange;

    private void Start()
    {
        Health        = entityData.baseHealth;
        CurrentHealth = Health;

        Mind  = entityData.mind;
        Body  = entityData.body;
        Focus = entityData.focus;
        
        m_healthChange.RaiseEvent( new[] { CurrentHealth, Health } );
    }

    public void TakeDamage( int damage )
    {
        CurrentHealth -= damage;
        m_healthChange.RaiseEvent( new[] { CurrentHealth, Health } );
    }

    public void FixedUpdate()
    {
        if ( entityData ) entityData.position = transform.position;
    }
}

public enum StatType
{
    Focus,
    Mind,
    Body,
    Health
}